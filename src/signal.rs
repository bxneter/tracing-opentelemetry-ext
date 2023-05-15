use tokio::signal::ctrl_c;
use tokio::signal::unix::{signal, SignalKind};

pub async fn shutdown_signal() {
    let ctrl_c = async {
        ctrl_c().await.expect("Failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal(SignalKind::terminate()).expect("Failed to install signal handler").recv().await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    tracing::warn!("Signal received, starting graceful shutdown");
    opentelemetry::global::shutdown_tracer_provider();
}
