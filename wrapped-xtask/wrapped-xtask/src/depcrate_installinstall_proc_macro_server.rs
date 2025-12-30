// Generated macro for install_proc_macro_server (function)
macro_rules! Depcrate_installinstall_proc_macro_server {
() => {
// Module: crate::install
// Provides: {"install_proc_macro_server"}
// Dependencies: {}
fn install_proc_macro_server (sh : & Shell , opts : ProcMacroServerOpt) -> anyhow :: Result < () > { let profile = if opts . dev_rel { "dev-rel" } else { "release" } ; cmd ! (sh , "cargo +nightly install --path crates/proc-macro-srv-cli --profile={profile} --locked --force --features sysroot-abi") . run () ? ; Ok (()) }
};
}
