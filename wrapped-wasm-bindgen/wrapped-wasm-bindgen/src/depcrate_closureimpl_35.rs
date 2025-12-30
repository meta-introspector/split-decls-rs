// Generated macro for impl_35 (impl)
macro_rules! Depcrate_closureimpl_35 {
() => {
// Module: crate::closure
// Provides: {"impl_35"}
// Dependencies: {}
impl < T > IntoWasmAbi for & Closure < T > where T : WasmClosure + ? Sized , { type Abi = u32 ; fn into_abi (self) -> u32 { (& * self . js) . into_abi () } }
};
}
