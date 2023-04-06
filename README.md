# connection-profiler

This is a small project that allows to profile a postgres/YSQL connection.

This is done by creating a local copy of the [rust-postgres](https://github.com/sfackler/rust-postgres.git) crates, and patching some function to include timing.  

There are two mandatory parameters:   
`--url`/`-u`: the connection definition for connecting to a postgres compatible server, such as `host=192.168.66.80 port=5433 sslmode=disable user=yugabyte password=yugabyte`  
`--query`/ `-q`: the query to be performed. If you only want to profile the connection, enter "".

Optionally, you can specify:  
`--protocol`/`-p`: perform query via `simple` query protocol (default), `extended` query protocol, or `prepared`, which uses a prepared statement and the extended query protocol.  
`--repeat-sql`/`-r`: repeat SQL execution argument times (default: 1).  
`--repeat-connect`/`-n`: repeat connecting to specified URL (default: 1), then repeat SQL execution `--repeat-sql` time.

## Note on protocol implementation in Rust vs. JDBC
The postgres crate implements both the simple query protocol and the extended query protocol.

With the Rust language, a connection does not have a property that sets the protocol. Actually, the wire protocol is something independent from how the query is executed by the postgres backend.
In that sense, the way the query is executed is not really a protocol, but dependent on the 'commands' given (see postgres: backend/tcop/postgres.c:PostgresMain).

The simple query 'protocol' translates to command 'Q', and the extended query protocol translates to commands 'P', 'B' and 'E'.

With the extended query protocol, the Rust implementation is that the client sends a 'P' (parse) message and waits for the response, and then sends a message with 'B' (bind=plan) and 'E' (execute).
With the extended query protocol, the JDBC implementation sends the 'P', 'B' and 'E' commands in one go.

This means you should not compare the connection profiler extended query protocol figures with JDBC extended query protocol. It's better to use the simple query protocol for that.


#### For finding the changes in the patched rust-postgres crate
To find the changed entries in the patched postgres crate: search for FRITS (`find rust-postgres -exec grep -Hi FRITS {} \; 2>/dev/null`)
