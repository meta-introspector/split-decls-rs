// Generated macro for impl_42 (impl)
macro_rules! Depcrateimpl_42 {
() => {
// Module: crate
// Provides: {"impl_42"}
// Dependencies: {}
impl < T , F > GetSpan < T > for F where F : Fn (& T) -> tracing :: Span , { # [inline] fn span_for (& self , target : & T) -> tracing :: Span { (self) (target) } }
};
}
