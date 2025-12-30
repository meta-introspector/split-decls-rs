// Generated macro for js_value_vector_from_abi (function)
macro_rules! Depcrate_convert_implsjs_value_vector_from_abi {
() => {
// Module: crate::convert::impls
// Provides: {"js_value_vector_from_abi"}
// Dependencies: {}
# [doc = " # ⚠\u{fe0f} Unstable"] # [doc = ""] # [doc = " This is part of the internal [`convert`](crate::convert) module, **no"] # [doc = " stability guarantees** are provided. Use at your own risk. See its"] # [doc = " documentation for more details."] pub unsafe fn js_value_vector_from_abi < T : TryFromJsValue > (js : < Box < [JsValue] > as FromWasmAbi > :: Abi ,) -> Box < [T] > { let js_vals = < Vec < JsValue > as FromWasmAbi > :: from_abi (js) ; let mut result = Vec :: with_capacity (js_vals . len ()) ; for value in js_vals { result . push (T :: try_from_js_value (value) . expect_throw ("array contains a value of the wrong type") ,) ; } result . into_boxed_slice () }
};
}
