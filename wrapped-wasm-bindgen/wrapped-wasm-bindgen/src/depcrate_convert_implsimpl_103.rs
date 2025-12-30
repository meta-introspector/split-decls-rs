// Generated macro for impl_103 (impl)
macro_rules! Depcrate_convert_implsimpl_103 {
() => {
// Module: crate::convert::impls
// Provides: {"impl_103"}
// Dependencies: {}
impl RefFromWasmAbi for JsValue { type Abi = u32 ; type Anchor = ManuallyDrop < JsValue > ; # [inline] unsafe fn ref_from_abi (js : u32) -> Self :: Anchor { ManuallyDrop :: new (JsValue :: _new (js)) } }
};
}
