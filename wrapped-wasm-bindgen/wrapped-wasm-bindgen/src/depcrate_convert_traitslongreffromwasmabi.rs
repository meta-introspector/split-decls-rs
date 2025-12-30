// Generated macro for LongRefFromWasmAbi (trait)
macro_rules! Depcrate_convert_traitsLongRefFromWasmAbi {
() => {
// Module: crate::convert::traits
// Provides: {"LongRefFromWasmAbi"}
// Dependencies: {}
# [doc = " A version of the `RefFromWasmAbi` trait with the additional requirement"] # [doc = " that the reference must remain valid as long as the anchor isn't dropped."] # [doc = ""] # [doc = " This isn't the case for `JsValue`'s `RefFromWasmAbi` implementation. To"] # [doc = " avoid having to allocate a spot for the `JsValue` on the `JsValue` heap,"] # [doc = " the `JsValue` is instead pushed onto the `JsValue` stack, and popped off"] # [doc = " again after the function that the reference was passed to returns. So,"] # [doc = " `JsValue` has a different `LongRefFromWasmAbi` implementation that behaves"] # [doc = " the same as `FromWasmAbi`, putting the value on the heap."] # [doc = ""] # [doc = " This is needed for async functions, where the reference needs to be valid"] # [doc = " for the whole length of the `Future`, rather than the initial synchronous"] # [doc = " call."] # [doc = ""] # [doc = " 'long ref' is short for 'long-lived reference'."] # [doc = ""] # [doc = " # ⚠\u{fe0f} Unstable"] # [doc = ""] # [doc = " This is part of the internal [`convert`](crate::convert) module, **no"] # [doc = " stability guarantees** are provided. Use at your own risk. See its"] # [doc = " documentation for more details."] pub trait LongRefFromWasmAbi : WasmDescribe { # [doc = " Same as `RefFromWasmAbi::Abi`"] type Abi : WasmAbi ; # [doc = " Same as `RefFromWasmAbi::Anchor`"] type Anchor : Borrow < Self > ; # [doc = " Same as `RefFromWasmAbi::ref_from_abi`"] unsafe fn long_ref_from_abi (js : Self :: Abi) -> Self :: Anchor ; }
};
}
