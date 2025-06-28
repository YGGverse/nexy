mod config;
mod response;
mod server;
mod session;

fn main() -> anyhow::Result<()> {
    use clap::Parser;
    let c = config::Config::parse();
    let s = std::sync::Arc::new(session::Session::init(&c)?);
    for b in c.bind {
        println!("start server on `{b}`...");
        match std::net::TcpListener::bind(&b) {
            Ok(r) => {
                std::thread::spawn({
                    let s = s.clone();
                    move || server::start(r, &s)
                });
            }
            Err(e) => eprintln!("failed to start server on `{b}`: `{e}`"),
        }
    }
    std::thread::park();
    Ok(())
}
