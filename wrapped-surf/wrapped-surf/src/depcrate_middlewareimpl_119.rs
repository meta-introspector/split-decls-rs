// Generated macro for impl_119 (impl)
macro_rules! Depcrate_middlewareimpl_119 {
() => {
// Module: crate::middleware
// Provides: {"impl_119"}
// Dependencies: {}
# [async_trait] impl < F > Middleware for F where F : Send + Sync + 'static + for < 'a > Fn (Request , Client , Next < 'a >) -> BoxFuture < 'a , Result < Response > > , { async fn handle (& self , req : Request , client : Client , next : Next < '_ >) -> Result < Response > { (self) (req , client , next) . await } }
};
}
