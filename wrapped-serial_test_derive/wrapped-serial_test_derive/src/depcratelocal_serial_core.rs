// Generated macro for local_serial_core (function)
macro_rules! Depcratelocal_serial_core {
() => {
// Module: crate
// Provides: {"local_serial_core"}
// Dependencies: {}
fn local_serial_core (attr : proc_macro2 :: TokenStream , input : proc_macro2 :: TokenStream ,) -> proc_macro2 :: TokenStream { let config = get_config (attr) ; serial_setup (input , config , "local") }
};
}
