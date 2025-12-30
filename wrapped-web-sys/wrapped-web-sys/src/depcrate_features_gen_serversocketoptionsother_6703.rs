// Generated macro for other_6703 (other)
macro_rules! Depcrate_features_gen_ServerSocketOptionsother_6703 {
() => {
// Module: crate::features::gen_ServerSocketOptions
// Provides: {"other_6703"}
// Dependencies: {}
# [wasm_bindgen] extern "C" { # [wasm_bindgen (extends = :: js_sys :: Object , js_name = ServerSocketOptions)] # [derive (Debug , Clone , PartialEq , Eq)] # [doc = "The `ServerSocketOptions` dictionary."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `ServerSocketOptions`*"] pub type ServerSocketOptions ; # [cfg (feature = "TcpSocketBinaryType")] # [doc = "Get the `binaryType` field of this object."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `ServerSocketOptions`, `TcpSocketBinaryType`*"] # [wasm_bindgen (method , getter = "binaryType")] pub fn get_binary_type (this : & ServerSocketOptions) -> Option < TcpSocketBinaryType > ; # [cfg (feature = "TcpSocketBinaryType")] # [doc = "Change the `binaryType` field of this object."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `ServerSocketOptions`, `TcpSocketBinaryType`*"] # [wasm_bindgen (method , setter = "binaryType")] pub fn set_binary_type (this : & ServerSocketOptions , val : TcpSocketBinaryType) ; }
};
}
