# connection-profiler

This is a small project that allows to profile a postgres/YSQL connection.

There are two mandatory parameters:   
`--url`/`-u`: the connection definition for connecting to a postgres compatible server, such as `host=192.168.66.80 port=5433 sslmode=disable user=yugabyte password=yugabyte`  
`--query`/ `-q`: the query to be performed. If you only want to profile the connection, enter "".

Optionally, you can specify:  
`--protocol`/`-p`: perform query via `simple` query protocol (default) or `extended` query protocol.  
`--prepare`/`-e`: (only for extended query protocol) prepare query, use prepared query in bind and execute phases (not default).  
`--repeat-sql`/`-r`: repeat SQL execution argument times (default: 1).  
`--repeat-connect`/`-n`: repeat connecting to specified URL, then repeat repeat-sql time (default: 1).

