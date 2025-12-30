// Generated macro for impl_245 (impl)
macro_rules! Depcrate_predicate_formsimpl_245 {
() => {
// Module: crate::predicate_forms
// Provides: {"impl_245"}
// Dependencies: {}
impl fmt :: Display for PredicationMask { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . m . then (| | write ! (f , "m")) . transpose () ? ; self . x . then (| | write ! (f , "x")) . transpose () ? ; self . z . then (| | write ! (f , "z")) . transpose () . map (| _ | ()) } }
};
}
