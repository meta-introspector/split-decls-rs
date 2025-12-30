// Generated macro for VectorIntoWasmAbi (trait)
macro_rules! Depcrate_convert_traitsVectorIntoWasmAbi {
() => {
// Module: crate::convert::traits
// Provides: {"VectorIntoWasmAbi"}
// Dependencies: {}
# [doc = " Trait for element types to implement IntoWasmAbi for vectors of"] # [doc = " themselves."] # [doc = ""] # [doc = " # ⚠\u{fe0f} Unstable"] # [doc = ""] # [doc = " This is part of the internal [`convert`](crate::convert) module, **no"] # [doc = " stability guarantees** are provided. Use at your own risk. See its"] # [doc = " documentation for more details."] pub trait VectorIntoWasmAbi : WasmDescribeVector + Sized { type Abi : WasmAbi ; fn vector_into_abi (vector : Box < [Self] >) -> Self :: Abi ; }
};
}
