// Generated macro for impl_41 (impl)
macro_rules! Depcrate_attrimpl_41 {
() => {
// Module: crate::attr
// Provides: {"impl_41"}
// Dependencies: {}
impl ToTokens for Level { fn to_tokens (& self , tokens : & mut TokenStream) { match self { Level :: Trace => tokens . extend (quote ! (:: tracing :: Level :: TRACE)) , Level :: Debug => tokens . extend (quote ! (:: tracing :: Level :: DEBUG)) , Level :: Info => tokens . extend (quote ! (:: tracing :: Level :: INFO)) , Level :: Warn => tokens . extend (quote ! (:: tracing :: Level :: WARN)) , Level :: Error => tokens . extend (quote ! (:: tracing :: Level :: ERROR)) , Level :: Path (ref pat) => tokens . extend (quote ! (# pat)) , } } }
};
}
