// Generated macro for VectorFromWasmAbi (trait)
macro_rules! Depcrate_convert_traitsVectorFromWasmAbi {
() => {
// Module: crate::convert::traits
// Provides: {"VectorFromWasmAbi"}
// Dependencies: {}
# [doc = " Trait for element types to implement FromWasmAbi for vectors of"] # [doc = " themselves."] # [doc = ""] # [doc = " # ⚠\u{fe0f} Unstable"] # [doc = ""] # [doc = " This is part of the internal [`convert`](crate::convert) module, **no"] # [doc = " stability guarantees** are provided. Use at your own risk. See its"] # [doc = " documentation for more details."] pub trait VectorFromWasmAbi : WasmDescribeVector + Sized { type Abi : WasmAbi ; unsafe fn vector_from_abi (js : Self :: Abi) -> Box < [Self] > ; }
};
}
