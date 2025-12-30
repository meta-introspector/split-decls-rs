// Generated macro for impl_434 (impl)
macro_rules! Depcrate_runtimeimpl_434 {
() => {
// Module: crate::runtime
// Provides: {"impl_434"}
// Dependencies: {}
impl std :: fmt :: Debug for Runtime { fn fmt (& self , fmt : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { fmt . debug_struct ("Runtime") . field ("revisions" , & self . revisions) . field ("revision_canceled" , & self . revision_canceled) . field ("dependency_graph" , & self . dependency_graph) . finish () } }
};
}
