// Generated macro for impl_123 (impl)
macro_rules! Depcrate_middlewareimpl_123 {
() => {
// Module: crate::middleware
// Provides: {"impl_123"}
// Dependencies: {}
impl < 'a > Next < 'a > { # [doc = " Create a new instance"] pub fn new (next : & 'a [Arc < dyn Middleware >] , endpoint : & 'a (dyn (Fn (Request , Client) -> BoxFuture < 'static , Result < Response > >) + Send + Sync + 'static) ,) -> Self { Self { endpoint , next_middleware : next , } } # [doc = " Asynchronously execute the remaining middleware chain."] pub fn run (mut self , req : Request , client : Client) -> BoxFuture < 'a , Result < Response > > { if let Some ((current , next)) = self . next_middleware . split_first () { self . next_middleware = next ; current . handle (req , client , self) } else { (self . endpoint) (req , client) } } }
};
}
