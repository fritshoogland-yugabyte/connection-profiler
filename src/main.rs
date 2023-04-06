//use postgres::{Client, NoTls};
use postgres::Client;
use openssl::ssl::{SslConnector, SslMethod, SslVerifyMode};
use postgres_openssl::MakeTlsConnector;
use std::process;
use std::time::Instant;
use std::io::stdin;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opts {
    /// postgres URL ("host=<host> port=<port> user=<user> password=<password> [sslmode=<require|prefer|disable>] [dbname=<database>]")
    #[structopt(short, long)]
    url: String,
    /// query
    #[structopt(short, long)]
    query: String,
    /// query protocol type
    #[structopt(short, long, default_value = "simple", possible_values(&["simple", "extended", "prepared", "test"]))]
    protocol: String,
    /// repeat query
    #[structopt(short, long, default_value = "1")]
    repeat_sql: usize,
    /// repeat connection
    #[structopt(short = "n", long, default_value = "1")]
    repeat_connect: usize,
}

const PAUSE: bool = false;

fn main() {
    env_logger::init();

    let options = Opts::from_args();
    let mut connection_total = 0;
    let mut simple_total = 0;
    let mut prepare_total = 0;
    let mut extended_total = 0;

    for _nr in 1..=options.repeat_connect
    {
        // Create a database connection connection.
        let connection_start = Instant::now();
        let mut connbuilder = SslConnector::builder(SslMethod::tls()).unwrap();
        connbuilder.set_verify(SslVerifyMode::NONE);
        let connector = MakeTlsConnector::new(connbuilder.build());
        let mut client = Client::connect(&options.url, connector).unwrap_or_else( |e|
        {
            eprintln!("Error creating connection: {}", e);
            process::exit(1);
        });
        let connection_elapsed = connection_start.elapsed().as_micros();
        println!("{:40} {:10} us", "create_connection", connection_elapsed);
        connection_total += connection_elapsed;

        // Create a prepared statement if protocol is set to prepared.
        let query = if &options.protocol == "prepared"
        {
            let prepare_start = Instant::now();
            let query = client.prepare(&options.query).unwrap();
            let prepare_elapsed = prepare_start.elapsed().as_micros();
            prepare_total += prepare_elapsed;
            println!("{:40} {:10} us", "prepare statement", prepare_elapsed);
            Some(query)
        } else {
            None
        };

        println!("{}", '-'.to_string().repeat(60));
        if PAUSE
        {
            println!("Press enter to continue");
            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();
        };

        for inside_nr in 1..=options.repeat_sql
        {
            if !options.query.is_empty()
            {
                println!("run nr: {}", inside_nr);

                // Simple query protocol, alias 'Q'.
                if &options.protocol == "simple"
                {
                    let simple_query_start = Instant::now();
                    let result = client.simple_query(&options.query);
                    let simple_query_elapsed = simple_query_start.elapsed().as_micros();
                    simple_total += simple_query_elapsed;
                    let _ = result.unwrap_or_else(|e| {
                        println!("{}", e);
                        Vec::new()
                    });
                    println!("{:40} {:10} us", "total simple query", simple_query_elapsed);
                }

                // Extended query protocol, alias 'P', 'B' and 'E'.
                if &options.protocol == "extended" || &options.protocol == "prepared"
                {
                    let extended_protocol_query_start = Instant::now();
                    let result = if &options.protocol == "prepared"
                    {
                        let q = query.clone().unwrap();
                        client.query(&q, &[])
                    } else {
                        client.query(&options.query, &[])
                    };

                    let extended_protocol_query_elapsed = extended_protocol_query_start.elapsed().as_micros();
                    extended_total += extended_protocol_query_elapsed;
                    let _ = result.unwrap_or_else(|e| {
                        println!("{}", e);
                        Vec::new()
                    });
                    println!("{:40} {:10} us", "total extended protocol", extended_protocol_query_elapsed);
                }

                if &options.protocol == "test"
                {
                    println!("execute");
                    let _ = client.execute(&options.query, &[]);
                }
            }
        }
        client.close().unwrap();
        println!("{}", '='.to_string().repeat(60));
    }
    println!("Average connection time: {:10}", connection_total/options.repeat_connect as u128);
    println!("Average prepare    time: {:10}", prepare_total/options.repeat_connect as u128);
    println!("Average simple     time: {:10}", simple_total/(options.repeat_connect as u128 * options.repeat_sql as u128));
    println!("Average extended   time: {:10}", extended_total/(options.repeat_connect as u128 * options.repeat_sql as u128));
}
