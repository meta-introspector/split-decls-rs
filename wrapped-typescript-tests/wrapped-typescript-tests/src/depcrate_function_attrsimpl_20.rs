// Generated macro for impl_20 (impl)
macro_rules! Depcrate_function_attrsimpl_20 {
() => {
// Module: crate::function_attrs
// Provides: {"impl_20"}
// Dependencies: {}
# [wasm_bindgen] impl HoldsNumber { # [doc = " Inner value"] # [wasm_bindgen (getter = "inner" , unchecked_return_type = "number")] pub fn get_inner (& self) -> JsValue { self . inner . clone () } # [doc = " Description for static_fn_with_attr"] # [wasm_bindgen (return_description = "returns an instance of HoldsNumber, holding arg1 if arg2 is undefined and holding arg2 if not")] pub fn static_fn_with_attr (# [wasm_bindgen (js_name = "firstArg" , param_description = "some number")] arg1 : u32 , # [wasm_bindgen (js_name = "secondArg" , unchecked_param_type = "number | undefined")] arg2 : JsValue ,) -> HoldsNumber { if arg2 . is_undefined () { HoldsNumber { inner : arg1 . into () } } else { HoldsNumber { inner : arg2 } } } # [doc = " Description for method_with_attr"] # [wasm_bindgen (unchecked_return_type = "number" , return_description = "returns arg1 if arg2 is true, or holding value of self if arg2 is undefined or false")] pub fn method_with_attr (& self , # [wasm_bindgen (js_name = "firstArg" , param_description = "some number")] arg1 : u32 , # [wasm_bindgen (js_name = "secondArg" , unchecked_param_type = "boolean | undefined")] arg2 : JsValue ,) -> JsValue { if arg2 . is_undefined () { self . inner . clone () } else if arg2 . is_truthy () { arg1 . into () } else { self . inner . clone () } } }
};
}
