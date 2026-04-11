use crate::{response::Response, session::Session};
use anyhow::Result;
use log::*;
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
        fn handle_err(connection: &Connection, message: String, request: Option<&str>) -> Vec<u8> {
            let bytes = connection.session.template.internal_server_error();
            error!(
                "{} > {} internal server error: `{message}`",
                connection.address.server, connection.address.client,
            );
            if let Some(ref access_log) = connection.session.access_log {
                access_log.clf(&connection.address.client, request, 1, bytes.len())
            }
            bytes
        }
        let mut b = [0; 1024]; // restrict max query size (@TODO unspecified)
        let response = match self.stream.read(&mut b) {
            Ok(request_size) => match std::str::from_utf8(&b[..request_size]) {
                Ok(request_string) => {
                    trace!(
                        "{} < {} incoming request: `{request_string}`",
                        self.address.server, self.address.client
                    );
                    match self.session.public.get(request_string) {
                        Ok(response) => match response {
                            Response::File(bytes) => {
                                trace!(
                                    "{} < {} response with file.",
                                    self.address.server, self.address.client,
                                );
                                if let Some(ref access_log) = self.session.access_log {
                                    access_log.clf(
                                        &self.address.client,
                                        Some(request_string),
                                        0,
                                        bytes.len(),
                                    )
                                }
                                bytes
                            }
                            Response::Directory {
                                data: ref s,
                                is_root,
                            } => {
                                let bytes = if is_root {
                                    trace!(
                                        "{} < {} response with root.",
                                        self.address.server, self.address.client,
                                    );
                                    self.session.template.welcome(Some(s))
                                } else {
                                    trace!(
                                        "{} < {} response with dir.",
                                        self.address.server, self.address.client,
                                    );
                                    self.session.template.index(Some(s))
                                };
                                if let Some(ref access_log) = self.session.access_log {
                                    access_log.clf(
                                        &self.address.client,
                                        Some(request_string),
                                        0,
                                        bytes.len(),
                                    )
                                }
                                bytes
                            }
                            Response::NotFound { message } => {
                                let bytes = self.session.template.not_found();
                                debug!(
                                    "{} < {} response object not found; reason: `{message}`.",
                                    self.address.server, self.address.client,
                                );
                                if let Some(ref access_log) = self.session.access_log {
                                    access_log.clf(
                                        &self.address.client,
                                        Some(request_string),
                                        1,
                                        bytes.len(),
                                    )
                                }
                                bytes
                            }
                            Response::InternalServerError { message } => {
                                handle_err(&self, message, Some(request_string))
                            }
                        },
                        Err(e) => handle_err(&self, e.to_string(), Some(request_string)),
                    }
                }
                Err(e) => handle_err(&self, e.to_string(), None),
            },
            Err(e) => handle_err(&self, e.to_string(), None),
        };
        // send response
        let result = self.stream.write_all(&response);
        if result.is_ok()
            || result.is_err_and(|e| {
                if matches!(e.kind(), ErrorKind::BrokenPipe | ErrorKind::ConnectionReset) {
                    true
                } else {
                    error!(
                        "{} > {} error sending response: `{e}`",
                        self.address.server, self.address.client
                    );
                    false
                }
            })
        {
            match self.stream.flush() {
                Ok(()) => trace!(
                    "{} > {} sent {} bytes of response.",
                    self.address.server,
                    self.address.client,
                    response.len()
                ),
                Err(e) => trace!(
                    "{} > {} sent {} bytes of response without flash: `{e}`.",
                    self.address.server,
                    self.address.client,
                    response.len()
                ),
            }
        }
        // try graceful shutdown
        match self.stream.shutdown(std::net::Shutdown::Both) {
            Ok(()) => trace!(
                "{} - {} connection closed by server.",
                self.address.server, self.address.client,
            ),
            Err(e) => warn!(
                "{} > {} failed to close connection: `{e}`",
                self.address.server, self.address.client,
            ),
        }
    }
}
