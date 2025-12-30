// Generated macro for impl_192 (impl)
macro_rules! Depcrate_convert_traitsimpl_192 {
() => {
// Module: crate::convert::traits
// Provides: {"impl_192"}
// Dependencies: {}
impl < T : WasmAbi > From < T > for WasmRet < T > { fn from (value : T) -> Self { let (prim1 , prim2 , prim3 , prim4) = value . split () ; Self { prim1 , prim2 , prim3 , prim4 , } } }
};
}
