// Generated macro for debug (module)
macro_rules! Depcrate_debugdebug {
() => {
// Module: crate::debug
// Provides: {"debug"}
// Dependencies: {}
# [cfg (feature = "debug_prints")] # [macro_use] # [allow (clippy :: module_inception)] mod debug { use std :: sync :: OnceLock ; # [doc = " If debugging is [`enabled`], print the format string on the error output."] macro_rules ! debug_print { ($ ($ arg : tt) *) => { { if $ crate :: debug :: enabled () { eprintln ! ($ ($ arg) *) } } } ; } # [doc = " Return whether debugging features are enabled in this execution."] # [cfg (debug_assertions)] pub fn enabled () -> bool { static ENABLED : OnceLock < bool > = OnceLock :: new () ; * ENABLED . get_or_init (| | std :: env :: var ("YAMLRUST2_DEBUG") . is_ok ()) } }
};
}
