use axum::body::Body;
use axum::http::Request;
use axum::response::Response;
use opentelemetry::propagation::TextMapPropagator;
use opentelemetry::sdk::propagation::TraceContextPropagator;
use opentelemetry_http::{HeaderExtractor, HeaderInjector};
use tracing::Span;
use tracing_opentelemetry::OpenTelemetrySpanExt;

pub fn update_otel_name(req: Request<Body>) -> Request<Body> {
    Span::current().record("otel.name", format!("{} {}", req.method(), req.uri()));
    req
}

pub fn inject_trace_context<T>(mut res: Response<T>) -> Response<T> {
    let ctx = Span::current().context();
    TraceContextPropagator::new().inject_context(&ctx, &mut HeaderInjector(res.headers_mut()));
    res
}

pub fn extract_trace_context<T>(req: &Request<T>) -> Span {
    let context = TraceContextPropagator::new().extract(&HeaderExtractor(req.headers()));
    let span = Span::current();
    span.set_parent(context);
    span
}
