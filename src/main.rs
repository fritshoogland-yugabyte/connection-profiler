use postgres::{Client, NoTls};
use std::process;
use std::time::Instant;
use std::io::stdin;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opts {
    /// postgres URL
    #[structopt(short, long)]
    url: String,
    /// query
    #[structopt(short, long)]
    query: String,
    /// query protocol type: simple or extended
    #[structopt(short, long, default_value = "simple", possible_values(&["simple", "extended"]))]
    protocol: String,
    /// prepare
    #[structopt(short = "e", long)]
    prepare: bool,
    /// repeat query
    #[structopt(short, long, default_value = "1")]
    repeat_sql: usize,
    /// repeat connection
    #[structopt(short = "n", long, default_value = "1")]
    repeat_connect: usize,
}

const PAUSE: bool = false;

fn main() {

    let options = Opts::from_args();
    let url: &str = &options.url;
    let query_text: &str = &options.query;
    let protocol: &str = &options.protocol;
    let prepare: bool = options.prepare;
    let repeat_sql: usize = options.repeat_sql;
    let repeat_connect: usize = options.repeat_connect;

    for _nr in 1..=repeat_connect
    {
        /*
         * Create a database connection connection.
         */
        let connection_start = Instant::now();
        let mut client = Client::connect(url, NoTls).unwrap_or_else( |e|
        {
            eprintln!("Error creating connection: {}", e);
            process::exit(1);
        });
        let connection_elapsed = connection_start.elapsed().as_micros();
        println!("{:40} {:10} us", "create_connection", connection_elapsed);

        /*
         * Create a prepared statement if PREPARE is true and TYPE is extended.
         */
        let query = if prepare && protocol == "extended"
        {
            let prepare_start = Instant::now();
            let query = client.prepare(query_text).unwrap();
            let prepare_elapsed = prepare_start.elapsed().as_micros();
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

        for inside_nr in 1..=repeat_sql
        {
            println!("run nr: {}", inside_nr);

            if !query_text.is_empty()
            {
                /*
                 * Simple query protocol, alias 'Q'.
                */
                if protocol == "simple"
                {
                    let simple_query_start = Instant::now();
                    let result = client.simple_query(query_text);
                    let simple_query_elapsed = simple_query_start.elapsed().as_micros();
                    let _ = result.unwrap_or_else(|e| {
                        println!("{}", e);
                        Vec::new()
                    });
                    println!("{:40} {:10} us", "total simple query", simple_query_elapsed);
                }

                /*
                 * Extended query protocol, alias 'P', 'B' and 'E'.
                 */
                if protocol == "extended"
                {
                    let extended_protocol_query_start = Instant::now();
                    let result = if prepare
                    {
                        let q = query.clone().unwrap();
                        client.query(&q, &[])
                    } else {
                        client.query(query_text, &[])
                    };

                    let extended_protocol_query_elapsed = extended_protocol_query_start.elapsed().as_micros();
                    let _ = result.unwrap_or_else(|e| {
                        println!("{}", e);
                        Vec::new()
                    });
                    println!("{:40} {:10} us", "total extended protocol", extended_protocol_query_elapsed);
                }
            }
        }

        client.close().unwrap();
        println!("{}", '='.to_string().repeat(60));
    }


}