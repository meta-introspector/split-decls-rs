// Generated macro for Next (struct)
macro_rules! Depcrate_middlewareNext {
() => {
// Module: crate::middleware
// Provides: {"Next"}
// Dependencies: {}
# [doc = " The remainder of a middleware chain, including the endpoint."] # [allow (missing_debug_implementations)] pub struct Next < 'a > { next_middleware : & 'a [Arc < dyn Middleware >] , endpoint : & 'a (dyn (Fn (Request , Client) -> BoxFuture < 'static , Result < Response > >) + Send + Sync + 'static) , }
};
}
