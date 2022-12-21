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

