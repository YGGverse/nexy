mod config;
mod response;
mod server;
mod session;

fn main() -> anyhow::Result<()> {
    if std::env::var("RUST_LOG").is_ok() {
        use tracing_subscriber::{EnvFilter, fmt::*};
        struct T;
        impl time::FormatTime for T {
            fn format_time(&self, w: &mut format::Writer<'_>) -> std::fmt::Result {
                write!(w, "{}", chrono::Local::now())
            }
        }
        fmt()
            .with_timer(T)
            .with_env_filter(EnvFilter::from_default_env())
            .init()
    }
    use clap::Parser;
    let c = config::Config::parse();
    let s = std::sync::Arc::new(session::Session::init(&c)?);
    for b in c.bind {
        log::info!("start server on `{b}`...");
        match std::net::TcpListener::bind(&b) {
            Ok(r) => {
                std::thread::spawn({
                    let s = s.clone();
                    move || server::start(r, &s)
                });
            }
            Err(e) => log::error!("failed to start server on `{b}`: `{e}`"),
        }
    }
    std::thread::park();
    Ok(())
}
