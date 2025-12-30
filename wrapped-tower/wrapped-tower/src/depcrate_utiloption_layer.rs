// Generated macro for option_layer (function)
macro_rules! Depcrate_utiloption_layer {
() => {
// Module: crate::util
// Provides: {"option_layer"}
// Dependencies: {}
# [doc = " Convert an `Option<Layer>` into a [`Layer`]."] # [doc = ""] # [doc = " ```"] # [doc = " # use std::time::Duration;"] # [doc = " # use tower::Service;"] # [doc = " # use tower::builder::ServiceBuilder;"] # [doc = " use tower::util::option_layer;"] # [doc = " # use tower::timeout::TimeoutLayer;"] # [doc = " # async fn wrap<S>(svc: S) where S: Service<(), Error = &'static str> + 'static + Send, S::Future: Send {"] # [doc = " # let timeout = Some(Duration::new(10, 0));"] # [doc = " // Layer to apply a timeout if configured"] # [doc = " let maybe_timeout = option_layer(timeout.map(TimeoutLayer::new));"] # [doc = ""] # [doc = " ServiceBuilder::new()"] # [doc = "     .layer(maybe_timeout)"] # [doc = "     .service(svc);"] # [doc = " # }"] # [doc = " ```"] # [doc = ""] # [doc = " [`Layer`]: crate::layer::Layer"] pub fn option_layer < L > (layer : Option < L >) -> Either < L , Identity > { if let Some (layer) = layer { Either :: Left (layer) } else { Either :: Right (Identity :: new ()) } }
};
}
