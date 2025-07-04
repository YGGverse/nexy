use crate::{response::Response, session::Session};
use anyhow::Result;
use std::{
    io::{ErrorKind, Read, Write},
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
                if let Some(ref request) = self.session.request {
                    request.add(&self.address.client, &q)
                }
                if self.session.clone().public.request(&q, |response| {
                    self.response(response).is_ok_and(|sent| {
                        t += sent;
                        if self.session.is_debug {
                            println!(
                                "[{}] > [{}] sent {sent} ({t} total) bytes response.",
                                self.address.server, self.address.client
                            )
                        };
                        true
                    })
                }) {
                    if let Some(ref a) = self.session.access_log {
                        a.clf(&self.address.client, Some(&q), 0, t)
                    }
                    self.shutdown()
                } else {
                    if let Some(ref a) = self.session.access_log {
                        a.clf(&self.address.client, Some(&q), 1, t)
                    }
                    if self.session.is_debug {
                        println!(
                            "[{}] - [{}] connection closed by client.",
                            self.address.server, self.address.client,
                        )
                    }
                }
            }
            Err(e) => match self.response(Response::InternalServerError {
                message: format!(
                    "[{}] < [{}] failed to handle incoming request: `{e}`",
                    self.address.server, self.address.client
                ),
                path: None,
                query: None,
            }) {
                Ok(sent) => {
                    t += sent;
                    if self.session.is_debug {
                        println!(
                            "[{}] > [{}] sent {sent} ({t} total) bytes response.",
                            self.address.server, self.address.client
                        )
                    };
                    if let Some(ref a) = self.session.access_log {
                        a.clf(&self.address.client, None, 2, t)
                    }
                    self.shutdown()
                }
                Err(e) => {
                    eprintln!(
                        "[{}] > [{}] handle request error: `{e}`",
                        self.address.server, self.address.client
                    );
                    if let Some(ref a) = self.session.access_log {
                        a.clf(&self.address.client, None, 1, t)
                    }
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

    fn response(&mut self, response: Response) -> std::io::Result<usize> {
        let data = match response {
            Response::File(b) => b,
            Response::Directory {
                query: q,
                data: ref s,
                is_root,
            } => {
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
            Response::InternalServerError {
                message,
                path,
                query,
            } => {
                eprintln!(
                    "[{}] > [{}] internal server error: `{message}` query: `{query:?}` path: `{:?}`",
                    self.address.server,
                    self.address.client,
                    path.map(|p| p.to_string_lossy().to_string()),
                );
                self.session.template.internal_server_error()
            }
            Response::AccessDenied {
                canonical,
                path,
                query,
            } => {
                eprintln!(
                    "[{}] < [{}] access denied: `{query}` (original: `{}` / canonical: `{}`)",
                    self.address.server,
                    self.address.client,
                    path.to_string_lossy(),
                    canonical.to_string_lossy()
                );
                self.session.template.access_denied()
            }
            Response::NotFound {
                message,
                path,
                query,
            } => {
                eprintln!(
                    "[{}] < [{}] not found: `{query}` (`{}`) reason: {message}",
                    self.address.server,
                    self.address.client,
                    path.to_string_lossy()
                );
                self.session.template.not_found()
            }
        };
        match self.stream.write_all(data) {
            Ok(()) => {
                self.stream.flush()?;
                Ok(data.len())
            }
            Err(e) => {
                // client may close the active connection unexpectedly, ignore some kinds
                if !matches!(e.kind(), ErrorKind::BrokenPipe | ErrorKind::ConnectionReset) {
                    eprintln!(
                        "[{}] > [{}] error sending response: `{e}`",
                        self.address.server, self.address.client
                    )
                }
                Err(e)
            }
        }
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
