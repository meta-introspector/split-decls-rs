// Generated macro for IntoWasmAbi (trait)
macro_rules! Depcrate_convert_traitsIntoWasmAbi {
() => {
// Module: crate::convert::traits
// Provides: {"IntoWasmAbi"}
// Dependencies: {}
# [doc = " A trait for anything that can be converted into a type that can cross the"] # [doc = " Wasm ABI directly, eg `u32` or `f64`."] # [doc = ""] # [doc = " This is the opposite operation as `FromWasmAbi` and `Ref[Mut]FromWasmAbi`."] # [doc = ""] # [doc = " # ⚠\u{fe0f} Unstable"] # [doc = ""] # [doc = " This is part of the internal [`convert`](crate::convert) module, **no"] # [doc = " stability guarantees** are provided. Use at your own risk. See its"] # [doc = " documentation for more details."] pub trait IntoWasmAbi : WasmDescribe { # [doc = " The Wasm ABI type that this converts into when crossing the ABI"] # [doc = " boundary."] type Abi : WasmAbi ; # [doc = " Convert `self` into `Self::Abi` so that it can be sent across the wasm"] # [doc = " ABI boundary."] fn into_abi (self) -> Self :: Abi ; }
};
}
