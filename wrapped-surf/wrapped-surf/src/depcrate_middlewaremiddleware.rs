// Generated macro for Middleware (trait)
macro_rules! Depcrate_middlewareMiddleware {
() => {
// Module: crate::middleware
// Provides: {"Middleware"}
// Dependencies: {}
# [doc = " Middleware that wraps around remaining middleware chain."] # [async_trait] pub trait Middleware : 'static + Send + Sync { # [doc = " Asynchronously handle the request, and return a response."] async fn handle (& self , req : Request , client : Client , next : Next < '_ >) -> Result < Response > ; }
};
}
