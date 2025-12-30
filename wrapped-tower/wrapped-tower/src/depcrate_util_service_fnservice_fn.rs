// Generated macro for service_fn (function)
macro_rules! Depcrate_util_service_fnservice_fn {
() => {
// Module: crate::util::service_fn
// Provides: {"service_fn"}
// Dependencies: {}
# [doc = " Returns a new [`ServiceFn`] with the given closure."] # [doc = ""] # [doc = " This lets you build a [`Service`] from an async function that returns a [`Result`]."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use tower::{service_fn, Service, ServiceExt, BoxError};"] # [doc = " # struct Request;"] # [doc = " # impl Request {"] # [doc = " #     fn new() -> Self { Self }"] # [doc = " # }"] # [doc = " # struct Response(&'static str);"] # [doc = " # impl Response {"] # [doc = " #     fn new(body: &'static str) -> Self {"] # [doc = " #         Self(body)"] # [doc = " #     }"] # [doc = " #     fn into_body(self) -> &'static str { self.0 }"] # [doc = " # }"] # [doc = ""] # [doc = " # #[tokio::main]"] # [doc = " # async fn main() -> Result<(), BoxError> {"] # [doc = " async fn handle(request: Request) -> Result<Response, BoxError> {"] # [doc = "     let response = Response::new(\"Hello, World!\");"] # [doc = "     Ok(response)"] # [doc = " }"] # [doc = ""] # [doc = " let mut service = service_fn(handle);"] # [doc = ""] # [doc = " let response = service"] # [doc = "     .ready()"] # [doc = "     .await?"] # [doc = "     .call(Request::new())"] # [doc = "     .await?;"] # [doc = ""] # [doc = " assert_eq!(\"Hello, World!\", response.into_body());"] # [doc = " #"] # [doc = " # Ok(())"] # [doc = " # }"] # [doc = " ```"] pub fn service_fn < T > (f : T) -> ServiceFn < T > { ServiceFn { f } }
};
}
