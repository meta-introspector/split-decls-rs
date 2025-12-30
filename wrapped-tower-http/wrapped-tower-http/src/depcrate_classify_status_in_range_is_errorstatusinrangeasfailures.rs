// Generated macro for StatusInRangeAsFailures (struct)
macro_rules! Depcrate_classify_status_in_range_is_errorStatusInRangeAsFailures {
() => {
// Module: crate::classify::status_in_range_is_error
// Provides: {"StatusInRangeAsFailures"}
// Dependencies: {}
# [doc = " Response classifier that considers responses with a status code within some range to be"] # [doc = " failures."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " A client with tracing where server errors _and_ client errors are considered failures."] # [doc = ""] # [doc = " ```no_run"] # [doc = " use tower_http::{trace::TraceLayer, classify::StatusInRangeAsFailures};"] # [doc = " use tower::{ServiceBuilder, Service, ServiceExt};"] # [doc = " use http::{Request, Method};"] # [doc = " use http_body_util::Full;"] # [doc = " use bytes::Bytes;"] # [doc = " use hyper_util::{rt::TokioExecutor, client::legacy::Client};"] # [doc = ""] # [doc = " # async fn foo() -> Result<(), tower::BoxError> {"] # [doc = " let classifier = StatusInRangeAsFailures::new(400..=599);"] # [doc = ""] # [doc = " let client = Client::builder(TokioExecutor::new()).build_http();"] # [doc = " let mut client = ServiceBuilder::new()"] # [doc = "     .layer(TraceLayer::new(classifier.into_make_classifier()))"] # [doc = "     .service(client);"] # [doc = ""] # [doc = " let request = Request::builder()"] # [doc = "     .method(Method::GET)"] # [doc = "     .uri(\"https://example.com\")"] # [doc = "     .body(Full::<Bytes>::default())"] # [doc = "     .unwrap();"] # [doc = ""] # [doc = " let response = client.ready().await?.call(request).await?;"] # [doc = " # Ok(())"] # [doc = " # }"] # [doc = " ```"] # [derive (Debug , Clone)] pub struct StatusInRangeAsFailures { range : RangeInclusive < u16 > , }
};
}
