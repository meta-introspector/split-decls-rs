// Generated macro for OptionIntoWasmAbi (trait)
macro_rules! Depcrate_convert_traitsOptionIntoWasmAbi {
() => {
// Module: crate::convert::traits
// Provides: {"OptionIntoWasmAbi"}
// Dependencies: {}
# [doc = " Indicates that this type can be passed to JS as `Option<Self>`."] # [doc = ""] # [doc = " This trait is used when implementing `IntoWasmAbi for Option<T>`."] # [doc = ""] # [doc = " # ⚠\u{fe0f} Unstable"] # [doc = ""] # [doc = " This is part of the internal [`convert`](crate::convert) module, **no"] # [doc = " stability guarantees** are provided. Use at your own risk. See its"] # [doc = " documentation for more details."] pub trait OptionIntoWasmAbi : IntoWasmAbi { # [doc = " Returns an ABI instance indicating \"none\", which JS will interpret as"] # [doc = " the `None` branch of this option."] # [doc = ""] # [doc = " It should be guaranteed that the `IntoWasmAbi` can never produce the ABI"] # [doc = " value returned here."] fn none () -> Self :: Abi ; }
};
}
