mod config;
mod response;
mod server;
mod session;

fn main() -> anyhow::Result<()> {
    use clap::Parser;
    let a = config::Config::parse();
    let s = std::sync::Arc::new(session::Session::init(&a)?);
    for b in a.bind {
        s.debug.info(&format!("start server on `{b}`..."));
        match std::net::TcpListener::bind(&b) {
            Ok(r) => {
                std::thread::spawn({
                    let s = s.clone();
                    move || server::start(r, &s)
                });
            }
            Err(e) => s
                .debug
                .error(&format!("failed to start server on `{b}`: `{e}`")),
        }
    }
    std::thread::park();
    Ok(())
}
