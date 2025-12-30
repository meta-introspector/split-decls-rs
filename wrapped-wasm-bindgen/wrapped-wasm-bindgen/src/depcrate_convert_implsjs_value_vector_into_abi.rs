// Generated macro for js_value_vector_into_abi (function)
macro_rules! Depcrate_convert_implsjs_value_vector_into_abi {
() => {
// Module: crate::convert::impls
// Provides: {"js_value_vector_into_abi"}
// Dependencies: {}
# [doc = " # ⚠\u{fe0f} Unstable"] # [doc = ""] # [doc = " This is part of the internal [`convert`](crate::convert) module, **no"] # [doc = " stability guarantees** are provided. Use at your own risk. See its"] # [doc = " documentation for more details."] pub fn js_value_vector_into_abi < T : Into < JsValue > > (vector : Box < [T] > ,) -> < Box < [JsValue] > as IntoWasmAbi > :: Abi { let js_vals : Box < [JsValue] > = vector . into_vec () . into_iter () . map (| x | x . into ()) . collect () ; js_vals . into_abi () }
};
}
