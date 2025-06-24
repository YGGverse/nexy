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
        match self.request() {
            Ok(q) => {
                self.session.debug.info(&format!(
                    "[{}] < [{}] request `{q}`...",
                    self.address.server, self.address.client
                ));
                self.session
                    .clone()
                    .storage
                    .request(&q, |r| self.response(r))
            }
            Err(e) => self.response(Response::InternalServerError(format!(
                "[{}] < [{}] failed to handle incoming request: `{e}`",
                self.address.server, self.address.client
            ))),
        }
        self.shutdown()
    }

    fn request(&mut self) -> Result<String> {
        let mut b = [0; 1024]; // @TODO unspecified len?
        let n = self.stream.read(&mut b)?;
        Ok(urlencoding::decode(std::str::from_utf8(&b[..n])?.trim())?.to_string())
    }

    fn response(&mut self, response: Response) {
        let bytes = match response {
            Response::File(b) => b,
            Response::Directory(s, is_root) => {
                &if is_root {
                    self.session.template.welcome(Some(&s))
                } else {
                    self.session.template.index(Some(&s))
                }
            }
            Response::InternalServerError(e) => {
                self.session.debug.error(&e);
                self.session.template.internal_server_error()
            }
            Response::AccessDenied(q) => {
                self.session.debug.error(&format!(
                    "[{}] < [{}] access to `{q}` denied.",
                    self.address.server, self.address.client
                ));
                self.session.template.access_denied()
            }
            Response::NotFound(q) => {
                self.session.debug.error(&format!(
                    "[{}] < [{}] requested resource `{q}` not found.",
                    self.address.server, self.address.client
                ));
                self.session.template.not_found()
            }
        };
        match self.stream.write_all(bytes) {
            Ok(()) => self.session.debug.info(&format!(
                "[{}] > [{}] sent {} bytes response.",
                self.address.server,
                self.address.client,
                bytes.len()
            )),
            Err(e) => self.session.debug.error(&format!(
                "[{}] ! [{}] failed to response: `{e}`",
                self.address.server, self.address.client,
            )),
        }
    }

    fn shutdown(self) {
        match self.stream.shutdown(std::net::Shutdown::Both) {
            Ok(()) => self.session.debug.info(&format!(
                "[{}] - [{}] connection closed by server.",
                self.address.server, self.address.client,
            )),
            Err(e) => self.session.debug.error(&format!(
                "[{}] > [{}] failed to close connection: `{e}`",
                self.address.server, self.address.client,
            )),
        }
    }
}
