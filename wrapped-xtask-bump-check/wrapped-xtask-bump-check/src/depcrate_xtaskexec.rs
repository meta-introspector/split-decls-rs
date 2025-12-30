// Generated macro for exec (function)
macro_rules! Depcrate_xtaskexec {
() => {
// Module: crate::xtask
// Provides: {"exec"}
// Dependencies: {}
pub fn exec (args : & clap :: ArgMatches , gctx : & mut cargo :: util :: GlobalContext) -> cargo :: CliResult { global_context_configure (gctx , args) ? ; bump_check (args , gctx) ? ; Ok (()) }
};
}
