// Generated macro for impl_107 (impl)
macro_rules! Depcrate_convert_implsimpl_107 {
() => {
// Module: crate::convert::impls
// Provides: {"impl_107"}
// Dependencies: {}
impl < T : OptionIntoWasmAbi > IntoWasmAbi for Option < T > { type Abi = T :: Abi ; # [inline] fn into_abi (self) -> T :: Abi { match self { None => T :: none () , Some (me) => me . into_abi () , } } }
};
}
