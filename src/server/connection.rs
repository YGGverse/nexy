use crate::{response::Response, session::Session};
use anyhow::Result;
use std::{
    io::{Read, Write},
    net::{SocketAddr, TcpStream},
    sync::Arc,
};

/// Parsed once endpoint addresses for this `stream`
struct Address {
    server: SocketAddr,
    client: SocketAddr,
}

/// Client/server connection with its features implementation
pub struct Connection {
    address: Address,
    session: Arc<Session>,
    stream: TcpStream,
}

impl Connection {
    pub fn init(session: &Arc<Session>, stream: TcpStream) -> Result<Self> {
        Ok(Self {
            address: Address {
                server: stream.local_addr()?,
                client: stream.peer_addr()?,
            },
            session: session.clone(),
            stream,
        })
    }

    pub fn handle(mut self) {
        let mut t = 0; // total bytes
        match self.request() {
            Ok(q) => {
                if self.session.is_debug {
                    println!(
                        "[{}] < [{}] incoming request: `{q}`",
                        self.address.server, self.address.client
                    )
                }
                if let Some(ref r) = self.session.request {
                    r.add(&self.address.client, &q)
                }
                if self
                    .session
                    .clone()
                    .public
                    .request(&q, |r| match self.response(r) {
                        Ok(sent) => {
                            t += sent;
                            if self.session.is_debug {
                                println!(
                                    "[{}] > [{}] sent {sent} ({t} total) bytes response.",
                                    self.address.server, self.address.client
                                )
                            };
                            true
                        }
                        Err(e) => {
                            eprintln!(
                                "[{}] > [{}] `{q}`: error sending response: `{e}`",
                                self.address.server, self.address.client
                            );
                            false
                        }
                    })
                {
                    self.session
                        .access_log
                        .clf(&self.address.client, Some(&q), 0, t);
                    self.shutdown()
                } else {
                    self.session
                        .access_log
                        .clf(&self.address.client, Some(&q), 1, t);
                }
            }
            Err(e) => match self.response(Response::InternalServerError(
                "",
                format!(
                    "[{}] < [{}] failed to handle incoming request: `{e}`",
                    self.address.server, self.address.client
                ),
            )) {
                Ok(sent) => {
                    t += sent;
                    if self.session.is_debug {
                        println!(
                            "[{}] > [{}] sent {sent} ({t} total) bytes response.",
                            self.address.server, self.address.client
                        )
                    };
                    self.session
                        .access_log
                        .clf(&self.address.client, None, 2, t);
                    self.shutdown()
                }
                Err(e) => {
                    eprintln!(
                        "[{}] > [{}] handle request error: `{e}`",
                        self.address.server, self.address.client
                    );
                    self.session
                        .access_log
                        .clf(&self.address.client, None, 1, t);
                    self.shutdown()
                }
            },
        }
    }

    fn request(&mut self) -> Result<String> {
        let mut b = [0; 1024]; // @TODO unspecified len?
        let n = self.stream.read(&mut b)?;
        Ok(urlencoding::decode(std::str::from_utf8(&b[..n])?.trim())?.to_string())
    }

    fn response(&mut self, response: Response) -> Result<usize> {
        let bytes = match response {
            Response::File(b) => b,
            Response::Directory(q, ref s, is_root) => {
                &if is_root {
                    self.session.template.welcome(
                        Some(s),
                        self.session.request.as_ref().map(|i| i.count()),
                        self.session.request.as_ref().map(|i| i.total(None)),
                    )
                } else {
                    self.session.template.index(
                        Some(s),
                        self.session.request.as_ref().map(|i| i.count()),
                        self.session.request.as_ref().map(|i| i.total(Some(q))),
                    )
                }
            }
            Response::InternalServerError(q, e) => {
                eprintln!(
                    "[{}] > [{}] `{q}`: internal server error: `{e}`",
                    self.address.server, self.address.client
                );
                self.session.template.internal_server_error()
            }
            Response::AccessDenied(q) => {
                eprintln!(
                    "[{}] < [{}] access to `{q}` denied.",
                    self.address.server, self.address.client
                );
                self.session.template.access_denied()
            }
            Response::NotFound(q) => {
                eprintln!(
                    "[{}] < [{}] requested resource `{q}` not found.",
                    self.address.server, self.address.client
                );
                self.session.template.not_found()
            }
        };
        self.stream.write_all(bytes)?;
        self.stream.flush()?;
        Ok(bytes.len())
    }

    fn shutdown(self) {
        match self.stream.shutdown(std::net::Shutdown::Both) {
            Ok(()) => {
                if self.session.is_debug {
                    println!(
                        "[{}] - [{}] connection closed by server.",
                        self.address.server, self.address.client,
                    )
                }
            }
            Err(e) => eprintln!(
                "[{}] > [{}] failed to close connection: `{e}`",
                self.address.server, self.address.client,
            ),
        }
    }
}
