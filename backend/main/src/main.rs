//! Minimal backend entry point.

use std::io::{self, Write};

fn main() -> io::Result<()> {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?
        .block_on(run())
}

async fn run() -> io::Result<()> {
    #[cfg(unix)]
    let (mut interrupt, mut terminate) = {
        use tokio::signal::unix::{SignalKind, signal};

        (
            signal(SignalKind::interrupt())?,
            signal(SignalKind::terminate())?,
        )
    };

    writeln!(io::stdout().lock(), "Hello world")?;

    #[cfg(unix)]
    tokio::select! {
        _ = interrupt.recv() => {}
        _ = terminate.recv() => {}
    }

    #[cfg(not(unix))]
    tokio::signal::ctrl_c().await?;

    Ok(())
}
