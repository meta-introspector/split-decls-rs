// Generated macro for RefFromWasmAbi (trait)
macro_rules! Depcrate_convert_traitsRefFromWasmAbi {
() => {
// Module: crate::convert::traits
// Provides: {"RefFromWasmAbi"}
// Dependencies: {}
# [doc = " A trait for anything that can be recovered as some sort of shared reference"] # [doc = " from the Wasm ABI boundary."] # [doc = ""] # [doc = " This is the shared reference variant of the opposite operation as"] # [doc = " `IntoWasmAbi`."] # [doc = ""] # [doc = " # ⚠\u{fe0f} Unstable"] # [doc = ""] # [doc = " This is part of the internal [`convert`](crate::convert) module, **no"] # [doc = " stability guarantees** are provided. Use at your own risk. See its"] # [doc = " documentation for more details."] pub trait RefFromWasmAbi : WasmDescribe { # [doc = " The Wasm ABI type references to `Self` are recovered from."] type Abi : WasmAbi ; # [doc = " The type that holds the reference to `Self` for the duration of the"] # [doc = " invocation of the function that has an `&Self` parameter. This is"] # [doc = " required to ensure that the lifetimes don't persist beyond one function"] # [doc = " call, and so that they remain anonymous."] type Anchor : Deref < Target = Self > ; # [doc = " Recover a `Self::Anchor` from `Self::Abi`."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " Same as `FromWasmAbi::from_abi`."] unsafe fn ref_from_abi (js : Self :: Abi) -> Self :: Anchor ; }
};
}
