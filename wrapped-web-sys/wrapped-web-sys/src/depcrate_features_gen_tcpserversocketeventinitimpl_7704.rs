// Generated macro for impl_7704 (impl)
macro_rules! Depcrate_features_gen_TcpServerSocketEventInitimpl_7704 {
() => {
// Module: crate::features::gen_TcpServerSocketEventInit
// Provides: {"impl_7704"}
// Dependencies: {}
impl TcpServerSocketEventInit { # [doc = "Construct a new `TcpServerSocketEventInit`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `TcpServerSocketEventInit`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_bubbles()` instead."] pub fn bubbles (& mut self , val : bool) -> & mut Self { self . set_bubbles (val) ; self } # [deprecated = "Use `set_cancelable()` instead."] pub fn cancelable (& mut self , val : bool) -> & mut Self { self . set_cancelable (val) ; self } # [deprecated = "Use `set_composed()` instead."] pub fn composed (& mut self , val : bool) -> & mut Self { self . set_composed (val) ; self } # [cfg (feature = "TcpSocket")] # [deprecated = "Use `set_socket()` instead."] pub fn socket (& mut self , val : Option < & TcpSocket >) -> & mut Self { self . set_socket (val) ; self } }
};
}
