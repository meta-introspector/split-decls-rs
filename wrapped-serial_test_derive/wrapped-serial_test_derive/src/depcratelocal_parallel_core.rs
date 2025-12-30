// Generated macro for local_parallel_core (function)
macro_rules! Depcratelocal_parallel_core {
() => {
// Module: crate
// Provides: {"local_parallel_core"}
// Dependencies: {}
fn local_parallel_core (attr : proc_macro2 :: TokenStream , input : proc_macro2 :: TokenStream ,) -> proc_macro2 :: TokenStream { let config = get_config (attr) ; parallel_setup (input , config , "local") }
};
}
