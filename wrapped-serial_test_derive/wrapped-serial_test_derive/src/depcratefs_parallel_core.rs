// Generated macro for fs_parallel_core (function)
macro_rules! Depcratefs_parallel_core {
() => {
// Module: crate
// Provides: {"fs_parallel_core"}
// Dependencies: {}
fn fs_parallel_core (attr : proc_macro2 :: TokenStream , input : proc_macro2 :: TokenStream ,) -> proc_macro2 :: TokenStream { let config = get_config (attr) ; parallel_setup (input , config , "fs") }
};
}
