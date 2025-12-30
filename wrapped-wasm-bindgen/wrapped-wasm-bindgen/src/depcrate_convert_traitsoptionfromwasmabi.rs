// Generated macro for OptionFromWasmAbi (trait)
macro_rules! Depcrate_convert_traitsOptionFromWasmAbi {
() => {
// Module: crate::convert::traits
// Provides: {"OptionFromWasmAbi"}
// Dependencies: {}
# [doc = " Indicates that this type can be received from JS as `Option<Self>`."] # [doc = ""] # [doc = " This trait is used when implementing `FromWasmAbi for Option<T>`."] # [doc = ""] # [doc = " # ⚠\u{fe0f} Unstable"] # [doc = ""] # [doc = " This is part of the internal [`convert`](crate::convert) module, **no"] # [doc = " stability guarantees** are provided. Use at your own risk. See its"] # [doc = " documentation for more details."] pub trait OptionFromWasmAbi : FromWasmAbi { # [doc = " Tests whether the argument is a \"none\" instance. If so it will be"] # [doc = " deserialized as `None`, and otherwise it will be passed to"] # [doc = " `FromWasmAbi`."] fn is_none (abi : & Self :: Abi) -> bool ; }
};
}
