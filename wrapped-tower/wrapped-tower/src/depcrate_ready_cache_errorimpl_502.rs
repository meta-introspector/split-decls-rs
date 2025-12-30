// Generated macro for impl_502 (impl)
macro_rules! Depcrate_ready_cache_errorimpl_502 {
() => {
// Module: crate::ready_cache::error
// Provides: {"impl_502"}
// Dependencies: {}
impl < K : std :: fmt :: Debug > std :: error :: Error for Failed < K > { fn source (& self) -> Option < & (dyn std :: error :: Error + 'static) > { Some (& * self . 1) } }
};
}
