// Generated macro for solana_program_init (function)
macro_rules! Depcratesolana_program_init {
() => {
// Module: crate
// Provides: {"solana_program_init"}
// Dependencies: {}
# [doc = " Initialize Javascript logging and panic handler"] # [wasm_bindgen] pub fn solana_program_init () { use std :: sync :: Once ; static INIT : Once = Once :: new () ; INIT . call_once (| | { std :: panic :: set_hook (Box :: new (console_error_panic_hook :: hook)) ; console_log :: init_with_level (Level :: Info) . unwrap () ; }) ; }
};
}
