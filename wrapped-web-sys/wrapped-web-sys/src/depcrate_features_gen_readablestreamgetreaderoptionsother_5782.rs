// Generated macro for other_5782 (other)
macro_rules! Depcrate_features_gen_ReadableStreamGetReaderOptionsother_5782 {
() => {
// Module: crate::features::gen_ReadableStreamGetReaderOptions
// Provides: {"other_5782"}
// Dependencies: {}
# [wasm_bindgen] extern "C" { # [wasm_bindgen (extends = :: js_sys :: Object , js_name = ReadableStreamGetReaderOptions)] # [derive (Debug , Clone , PartialEq , Eq)] # [doc = "The `ReadableStreamGetReaderOptions` dictionary."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `ReadableStreamGetReaderOptions`*"] pub type ReadableStreamGetReaderOptions ; # [cfg (feature = "ReadableStreamReaderMode")] # [doc = "Get the `mode` field of this object."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `ReadableStreamGetReaderOptions`, `ReadableStreamReaderMode`*"] # [wasm_bindgen (method , getter = "mode")] pub fn get_mode (this : & ReadableStreamGetReaderOptions) -> Option < ReadableStreamReaderMode > ; # [cfg (feature = "ReadableStreamReaderMode")] # [doc = "Change the `mode` field of this object."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `ReadableStreamGetReaderOptions`, `ReadableStreamReaderMode`*"] # [wasm_bindgen (method , setter = "mode")] pub fn set_mode (this : & ReadableStreamGetReaderOptions , val : ReadableStreamReaderMode) ; }
};
}
