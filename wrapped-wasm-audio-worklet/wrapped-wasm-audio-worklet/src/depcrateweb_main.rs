// Generated macro for web_main (function)
macro_rules! Depcrateweb_main {
() => {
// Module: crate
// Provides: {"web_main"}
// Dependencies: {}
# [wasm_bindgen] pub async fn web_main () { console_error_panic_hook :: set_once () ; let params : & 'static Params = Box :: leak (Box :: default ()) ; let mut osc = Oscillator :: new (params) ; let ctx = wasm_audio (Box :: new (move | buf | osc . process (buf))) . await . unwrap () ; create_gui (params , ctx) ; }
};
}
