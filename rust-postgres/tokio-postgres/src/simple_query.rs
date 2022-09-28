use crate::client::{InnerClient, Responses};
use crate::codec::FrontendMessage;
use crate::connection::RequestMessages;
use crate::{Error, SimpleQueryMessage, SimpleQueryRow};
use bytes::Bytes;
use fallible_iterator::FallibleIterator;
use futures::{ready, Stream};
use log::debug;
use pin_project_lite::pin_project;
use postgres_protocol::message::backend::Message;
use postgres_protocol::message::frontend;
use std::marker::PhantomPinned;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};

/// Information about a column of a single query row.
pub struct SimpleColumn {
    name: String,
}

impl SimpleColumn {
    pub(crate) fn new(name: String) -> SimpleColumn {
        SimpleColumn { name }
    }

    /// Returns the name of the column.
    pub fn name(&self) -> &str {
        &self.name
    }
}

pub async fn simple_query(client: &InnerClient, query: &str) -> Result<SimpleQueryStream, Error> {
    debug!("executing simple query: {}", query);

    let buf = encode(client, query)?;
    let responses = client.send(RequestMessages::Single(FrontendMessage::Raw(buf)))?;

    Ok(SimpleQueryStream {
        responses,
        columns: None,
        _p: PhantomPinned,
    })
}

pub async fn batch_execute(client: &InnerClient, query: &str) -> Result<(), Error> {
    debug!("executing statement batch: {}", query);

    let buf = encode(client, query)?;
    let mut responses = client.send(RequestMessages::Single(FrontendMessage::Raw(buf)))?;

    loop {
        match responses.next().await? {
            Message::ReadyForQuery(_) => return Ok(()),
            Message::CommandComplete(_)
            | Message::EmptyQueryResponse
            | Message::RowDescription(_)
            | Message::DataRow(_) => {}
            _ => return Err(Error::unexpected_message()),
        }
    }
}

fn encode(client: &InnerClient, query: &str) -> Result<Bytes, Error> {
    client.with_buf(|buf| {
        frontend::query(query, buf).map_err(Error::encode)?;
        Ok(buf.split().freeze())
    })
}

pin_project! {
    /// A stream of simple query results.
    pub struct SimpleQueryStream {
        responses: Responses,
        columns: Option<Arc<[SimpleColumn]>>,
        #[pin]
        _p: PhantomPinned,
    }
}

impl Stream for SimpleQueryStream {
    type Item = Result<SimpleQueryMessage, Error>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.project();
        loop {
            match ready!(this.responses.poll_next(cx)?) {
                Message::CommandComplete(body) => {
                    let rows = body
                        .tag()
                        .map_err(Error::parse)?
                        .rsplit(' ')
                        .next()
                        .unwrap()
                        .parse()
                        .unwrap_or(0);
                    return Poll::Ready(Some(Ok(SimpleQueryMessage::CommandComplete(rows))));
                }
                Message::EmptyQueryResponse => {
                    return Poll::Ready(Some(Ok(SimpleQueryMessage::CommandComplete(0))));
                }
                Message::RowDescription(body) => {
                    let columns = body
                        .fields()
                        .map(|f| Ok(SimpleColumn::new(f.name().to_string())))
                        .collect::<Vec<_>>()
                        .map_err(Error::parse)?
                        .into();

                    *this.columns = Some(columns);
                }
                Message::DataRow(body) => {
                    let row = match &this.columns {
                        Some(columns) => SimpleQueryRow::new(columns.clone(), body)?,
                        None => return Poll::Ready(Some(Err(Error::unexpected_message()))),
                    };
                    return Poll::Ready(Some(Ok(SimpleQueryMessage::Row(row))));
                }
                Message::ReadyForQuery(_) => return Poll::Ready(None),
                _ => return Poll::Ready(Some(Err(Error::unexpected_message()))),
            }
        }
    }
}
