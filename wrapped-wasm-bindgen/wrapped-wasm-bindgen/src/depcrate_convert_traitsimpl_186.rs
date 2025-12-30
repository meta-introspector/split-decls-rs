// Generated macro for impl_186 (impl)
macro_rules! Depcrate_convert_traitsimpl_186 {
() => {
// Module: crate::convert::traits
// Provides: {"impl_186"}
// Dependencies: {}
impl < T : IntoWasmAbi > ReturnWasmAbi for T { type Abi = T :: Abi ; # [inline] fn return_abi (self) -> Self :: Abi { self . into_abi () } }
};
}
