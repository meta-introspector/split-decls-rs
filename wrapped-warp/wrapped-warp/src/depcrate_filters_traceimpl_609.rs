// Generated macro for impl_609 (impl)
macro_rules! Depcrate_filters_traceimpl_609 {
() => {
// Module: crate::filters::trace
// Provides: {"impl_609"}
// Dependencies: {}
impl < FN , F > WrapSealed < F > for Trace < FN > where FN : Fn (Info < '_ >) -> Span + Clone + Send , F : Filter + Clone + Send , F :: Extract : Reply , F :: Error : IsReject , { type Wrapped = WithTrace < FN , F > ; fn wrap (& self , filter : F) -> Self :: Wrapped { WithTrace { filter , trace : self . clone () , } } }
};
}
