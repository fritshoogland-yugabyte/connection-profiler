use crate::connection::ConnectionRef;
use fallible_iterator::FallibleIterator;
use futures::StreamExt;
use std::pin::Pin;
use tokio_postgres::{Error, Row, RowStream};

/// The iterator returned by `query_raw`.
pub struct RowIter<'a> {
    connection: ConnectionRef<'a>,
    it: Pin<Box<RowStream>>,
}

impl<'a> RowIter<'a> {
    pub(crate) fn new(connection: ConnectionRef<'a>, stream: RowStream) -> RowIter<'a> {
        RowIter {
            connection,
            it: Box::pin(stream),
        }
    }
}

impl FallibleIterator for RowIter<'_> {
    type Item = Row;
    type Error = Error;

    fn next(&mut self) -> Result<Option<Row>, Error> {
        let it = &mut self.it;
        self.connection
            .block_on(async { it.next().await.transpose() })
    }
}
