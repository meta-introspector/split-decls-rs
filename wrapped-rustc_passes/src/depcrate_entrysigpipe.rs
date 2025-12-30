// Generated macro for sigpipe (function)
macro_rules! Depcrate_entrysigpipe {
() => {
// Module: crate::entry
// Provides: {"sigpipe"}
// Dependencies: {}
fn sigpipe (tcx : TyCtxt < '_ >) -> u8 { match tcx . sess . opts . unstable_opts . on_broken_pipe { rustc_target :: spec :: OnBrokenPipe :: Default => sigpipe :: DEFAULT , rustc_target :: spec :: OnBrokenPipe :: Kill => sigpipe :: SIG_DFL , rustc_target :: spec :: OnBrokenPipe :: Error => sigpipe :: SIG_IGN , rustc_target :: spec :: OnBrokenPipe :: Inherit => sigpipe :: INHERIT , } }
};
}
