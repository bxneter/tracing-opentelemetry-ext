use opentelemetry::propagation::{Injector, TextMapPropagator};
use opentelemetry::sdk::propagation::TraceContextPropagator;
use std::str::FromStr;
use tonic::metadata::{AsciiMetadataKey, AsciiMetadataValue, MetadataMap};
use tonic::service::Interceptor;
use tracing::Span;
use tracing_opentelemetry::OpenTelemetrySpanExt;

#[derive(Clone)]
pub struct TonicTraceInterceptor;

impl Interceptor for TonicTraceInterceptor {
    fn call(&mut self, mut req: tonic::Request<()>) -> tonic::Result<tonic::Request<()>> {
        let ctx = Span::current().context();
        TraceContextPropagator::new().inject_context(&ctx, &mut TonicMetadataInjector(req.metadata_mut()));
        Ok(req)
    }
}

struct TonicMetadataInjector<'a>(&'a mut MetadataMap);

impl Injector for TonicMetadataInjector<'_> {
    fn set(&mut self, key: &str, value: String) {
        if let (Ok(key), Ok(value)) = (AsciiMetadataKey::from_str(key), AsciiMetadataValue::try_from(&value)) {
            self.0.insert(key, value);
        }
    }
}
