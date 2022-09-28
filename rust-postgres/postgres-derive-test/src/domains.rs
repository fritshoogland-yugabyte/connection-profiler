use crate::test_type;
use postgres::{Client, NoTls};
use postgres_types::{FromSql, ToSql, WrongType};
use std::error::Error;

#[test]
fn defaults() {
    #[derive(FromSql, ToSql, Debug, PartialEq)]
    struct SessionId(Vec<u8>);

    let mut conn = Client::connect("user=postgres host=localhost port=5433", NoTls).unwrap();
    conn.execute(
        "CREATE DOMAIN pg_temp.\"SessionId\" AS bytea CHECK(octet_length(VALUE) = 16);",
        &[],
    )
    .unwrap();

    test_type(
        &mut conn,
        "\"SessionId\"",
        &[(
            SessionId(b"0123456789abcdef".to_vec()),
            "'0123456789abcdef'",
        )],
    );
}

#[test]
fn name_overrides() {
    #[derive(FromSql, ToSql, Debug, PartialEq)]
    #[postgres(name = "session_id")]
    struct SessionId(Vec<u8>);

    let mut conn = Client::connect("user=postgres host=localhost port=5433", NoTls).unwrap();
    conn.execute(
        "CREATE DOMAIN pg_temp.session_id AS bytea CHECK(octet_length(VALUE) = 16);",
        &[],
    )
    .unwrap();

    test_type(
        &mut conn,
        "session_id",
        &[(
            SessionId(b"0123456789abcdef".to_vec()),
            "'0123456789abcdef'",
        )],
    );
}

#[test]
fn wrong_name() {
    #[derive(FromSql, ToSql, Debug, PartialEq)]
    struct SessionId(Vec<u8>);

    let mut conn = Client::connect("user=postgres host=localhost port=5433", NoTls).unwrap();
    conn.execute(
        "CREATE DOMAIN pg_temp.session_id AS bytea CHECK(octet_length(VALUE) = 16);",
        &[],
    )
    .unwrap();

    let err = conn
        .execute("SELECT $1::session_id", &[&SessionId(vec![])])
        .unwrap_err();
    assert!(err.source().unwrap().is::<WrongType>());
}

#[test]
fn wrong_type() {
    #[derive(FromSql, ToSql, Debug, PartialEq)]
    #[postgres(name = "session_id")]
    struct SessionId(i32);

    let mut conn = Client::connect("user=postgres host=localhost port=5433", NoTls).unwrap();
    conn.execute(
        "CREATE DOMAIN pg_temp.session_id AS bytea CHECK(octet_length(VALUE) = 16);",
        &[],
    )
    .unwrap();

    let err = conn
        .execute("SELECT $1::session_id", &[&SessionId(0)])
        .unwrap_err();
    assert!(err.source().unwrap().is::<WrongType>());
}

#[test]
fn domain_in_composite() {
    #[derive(FromSql, ToSql, Debug, PartialEq)]
    #[postgres(name = "domain")]
    struct Domain(String);

    #[derive(FromSql, ToSql, Debug, PartialEq)]
    #[postgres(name = "composite")]
    struct Composite {
        domain: Domain,
    }

    let mut conn = Client::connect("user=postgres host=localhost port=5433", NoTls).unwrap();
    conn.batch_execute(
        "
            CREATE DOMAIN pg_temp.domain AS TEXT;\
            CREATE TYPE pg_temp.composite AS (
                domain domain
            );
        ",
    )
    .unwrap();

    test_type(
        &mut conn,
        "composite",
        &[(
            Composite {
                domain: Domain("hello".to_string()),
            },
            "ROW('hello')",
        )],
    );
}
