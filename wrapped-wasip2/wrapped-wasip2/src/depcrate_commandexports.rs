// Generated macro for exports (module)
macro_rules! Depcrate_commandexports {
() => {
// Module: crate::command
// Provides: {"exports"}
// Dependencies: {}
# [rustfmt :: skip] # [allow (dead_code , clippy :: all)] pub mod exports { pub mod wasi { pub mod cli { # [allow (dead_code , async_fn_in_trait , unused_imports , clippy :: all)] pub mod run { # [used] # [doc (hidden)] static __FORCE_SECTION_REF : fn () = super :: super :: super :: super :: __link_custom_section_describing_imports ; use super :: super :: super :: super :: _rt ; # [doc (hidden)] # [allow (non_snake_case , unused_unsafe)] pub unsafe fn _export_run_cabi < T : Guest > () -> i32 { unsafe { # [cfg (target_arch = "wasm32")] _rt :: run_ctors_once () ; let result0 = { T :: run () } ; let result1 = match result0 { Ok (_) => 0i32 , Err (_) => 1i32 , } ; result1 } } pub trait Guest { # [doc = " Run the program."] # [allow (async_fn_in_trait)] fn run () -> Result < () , () > ; } # [doc (hidden)] # [macro_export] macro_rules ! __export_wasi_cli_run_0_2_4_cabi { ($ ty : ident with_types_in $ ($ path_to_types : tt) *) => { const _ : () = { # [unsafe (export_name = "wasi:cli/run@0.2.4#run")] unsafe extern "C" fn export_run () -> i32 { unsafe { $ ($ path_to_types) *:: _export_run_cabi ::<$ ty > () } } } ; } ; } # [doc (hidden)] pub use __export_wasi_cli_run_0_2_4_cabi ; } } } }
};
}
