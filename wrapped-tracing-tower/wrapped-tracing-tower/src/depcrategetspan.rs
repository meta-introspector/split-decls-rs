// Generated macro for GetSpan (trait)
macro_rules! DepcrateGetSpan {
() => {
// Module: crate
// Provides: {"GetSpan"}
// Dependencies: {}
pub trait GetSpan < T > : crate :: sealed :: Sealed < T > { fn span_for (& self , target : & T) -> tracing :: Span ; }
};
}
