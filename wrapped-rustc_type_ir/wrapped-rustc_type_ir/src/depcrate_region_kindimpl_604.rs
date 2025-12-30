// Generated macro for impl_604 (impl)
macro_rules! Depcrate_region_kindimpl_604 {
() => {
// Module: crate::region_kind
// Provides: {"impl_604"}
// Dependencies: {}
impl < I : Interner > fmt :: Debug for RegionKind < I > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { ReEarlyParam (data) => write ! (f , "{data:?}") , ReBound (binder_id , bound_region) => { write ! (f , "'") ? ; crate :: debug_bound_var (f , * binder_id , bound_region) } ReLateParam (fr) => write ! (f , "{fr:?}") , ReStatic => f . write_str ("'static") , ReVar (vid) => write ! (f , "{vid:?}") , RePlaceholder (placeholder) => write ! (f , "'{placeholder:?}") , ReErased => f . write_str ("'{erased}") , ReError (_) => f . write_str ("'{region error}") , } } }
};
}
