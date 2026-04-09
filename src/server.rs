mod connection;

use crate::session::Session;
use connection::Connection;
use log::*;
use std::{net::TcpListener, sync::Arc, thread};

pub fn start(server: TcpListener, session: &Arc<Session>) {
    for i in server.incoming() {
        match i {
            Ok(stream) => {
                thread::spawn({
                    let session = session.clone();
                    move || match Connection::init(&session, stream) {
                        Ok(connection) => connection.handle(),
                        Err(e) => error!("failed to init connection: `{e}`"),
                    }
                });
            }
            Err(e) => error!("failed to accept incoming connection: `{e}`"),
        }
    }
}
