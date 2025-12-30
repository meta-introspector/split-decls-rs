// Generated macro for impl_194 (impl)
macro_rules! Depcrate_unstable_convert_stable_abiimpl_194 {
() => {
// Module: crate::unstable::convert::stable::abi
// Provides: {"impl_194"}
// Dependencies: {}
impl < 'tcx > Stable < 'tcx > for callconv :: PassMode { type T = PassMode ; fn stable (& self , _ : & mut Tables < '_ , BridgeTys > , _ : & CompilerCtxt < '_ , BridgeTys >) -> Self :: T { match self { callconv :: PassMode :: Ignore => PassMode :: Ignore , callconv :: PassMode :: Direct (attr) => PassMode :: Direct (opaque (attr)) , callconv :: PassMode :: Pair (first , second) => { PassMode :: Pair (opaque (first) , opaque (second)) } callconv :: PassMode :: Cast { pad_i32 , cast } => { PassMode :: Cast { pad_i32 : * pad_i32 , cast : opaque (cast) } } callconv :: PassMode :: Indirect { attrs , meta_attrs , on_stack } => PassMode :: Indirect { attrs : opaque (attrs) , meta_attrs : opaque (meta_attrs) , on_stack : * on_stack , } , } } }
};
}
