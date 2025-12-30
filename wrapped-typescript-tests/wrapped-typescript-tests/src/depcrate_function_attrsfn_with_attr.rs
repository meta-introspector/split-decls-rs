// Generated macro for fn_with_attr (function)
macro_rules! Depcrate_function_attrsfn_with_attr {
() => {
// Module: crate::function_attrs
// Provides: {"fn_with_attr"}
// Dependencies: {}
# [doc = " Description for fn_with_attr"] # [wasm_bindgen (unchecked_return_type = "number" , return_description = "returns 1 if arg2 is true, or arg1 if arg2 is undefined or false")] pub async fn fn_with_attr (# [wasm_bindgen (js_name = "firstArg" , param_description = "some number")] arg1 : u32 , # [wasm_bindgen (js_name = "secondArg" , unchecked_param_type = "boolean | undefined")] arg2 : JsValue ,) -> Result < JsValue , JsValue > { if arg2 . is_undefined () { Ok (arg1 . into ()) } else if arg2 . is_truthy () { Ok (1u32 . into ()) } else { Ok (arg1 . into ()) } }
};
}
