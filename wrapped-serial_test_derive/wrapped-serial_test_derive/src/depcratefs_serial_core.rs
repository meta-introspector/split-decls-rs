// Generated macro for fs_serial_core (function)
macro_rules! Depcratefs_serial_core {
() => {
// Module: crate
// Provides: {"fs_serial_core"}
// Dependencies: {}
fn fs_serial_core (attr : proc_macro2 :: TokenStream , input : proc_macro2 :: TokenStream ,) -> proc_macro2 :: TokenStream { let config = get_config (attr) ; serial_setup (input , config , "fs") }
};
}
