pub mod http;
pub mod macros;
pub mod masking;
pub mod signal;
pub mod tonic;
use axum_tracing_opentelemetry::otlp::{identity, init_tracer};
use opentelemetry::sdk::propagation::TraceContextPropagator;
use opentelemetry::sdk::Resource;
use opentelemetry::{global, KeyValue};
pub use tower::ServiceBuilder;
use tracing::subscriber::{set_global_default, SetGlobalDefaultError};
use tracing_opentelemetry::layer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{fmt, registry, EnvFilter};

pub fn init_tracing(service_name: &String, environment_name: &String) -> Result<(), SetGlobalDefaultError> {
    // Activate trace context propagation
    global::set_text_map_propagator(TraceContextPropagator::new());

    let service_name = KeyValue::new("service.name", service_name.to_string());
    let environment_name = KeyValue::new("deployment.environment", environment_name.to_string());
    let otel_resource = Resource::new(vec![service_name, environment_name]);
    let otel_tracer = init_tracer(otel_resource, identity).unwrap();
    let otel_layer = layer().with_tracer(otel_tracer);
    let otel_filter = EnvFilter::builder().from_env_lossy();
    let otel_stdout = fmt::layer().pretty().compact().with_target(false);
    let subscriber = registry().with(otel_filter).with(otel_stdout).with(otel_layer);

    set_global_default(subscriber)
}
