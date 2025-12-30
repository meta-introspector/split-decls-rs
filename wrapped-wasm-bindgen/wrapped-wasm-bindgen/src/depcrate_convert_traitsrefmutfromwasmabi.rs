// Generated macro for RefMutFromWasmAbi (trait)
macro_rules! Depcrate_convert_traitsRefMutFromWasmAbi {
() => {
// Module: crate::convert::traits
// Provides: {"RefMutFromWasmAbi"}
// Dependencies: {}
# [doc = " Dual of the `RefFromWasmAbi` trait, except for mutable references."] # [doc = ""] # [doc = " # ⚠\u{fe0f} Unstable"] # [doc = ""] # [doc = " This is part of the internal [`convert`](crate::convert) module, **no"] # [doc = " stability guarantees** are provided. Use at your own risk. See its"] # [doc = " documentation for more details."] pub trait RefMutFromWasmAbi : WasmDescribe { # [doc = " Same as `RefFromWasmAbi::Abi`"] type Abi : WasmAbi ; # [doc = " Same as `RefFromWasmAbi::Anchor`"] type Anchor : DerefMut < Target = Self > ; # [doc = " Same as `RefFromWasmAbi::ref_from_abi`"] unsafe fn ref_mut_from_abi (js : Self :: Abi) -> Self :: Anchor ; }
};
}
