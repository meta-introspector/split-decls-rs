// Generated macro for ReturnWasmAbi (trait)
macro_rules! Depcrate_convert_traitsReturnWasmAbi {
() => {
// Module: crate::convert::traits
// Provides: {"ReturnWasmAbi"}
// Dependencies: {}
# [doc = " A trait representing how to interpret the return value of a function for"] # [doc = " the Wasm ABI."] # [doc = ""] # [doc = " This is very similar to the `IntoWasmAbi` trait and in fact has a blanket"] # [doc = " implementation for all implementors of the `IntoWasmAbi`. The primary use"] # [doc = " case of this trait is to enable functions to return `Result`, interpreting"] # [doc = " an error as \"rethrow this to JS\""] # [doc = ""] # [doc = " # ⚠\u{fe0f} Unstable"] # [doc = ""] # [doc = " This is part of the internal [`convert`](crate::convert) module, **no"] # [doc = " stability guarantees** are provided. Use at your own risk. See its"] # [doc = " documentation for more details."] pub trait ReturnWasmAbi : WasmDescribe { # [doc = " Same as `IntoWasmAbi::Abi`"] type Abi : WasmAbi ; # [doc = " Same as `IntoWasmAbi::into_abi`, except that it may throw and never"] # [doc = " return in the case of `Err`."] fn return_abi (self) -> Self :: Abi ; }
};
}
