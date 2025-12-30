// Generated macro for impl_132 (impl)
macro_rules! Depcrate_low_level_siginfoimpl_132 {
() => {
// Module: crate::low_level::siginfo
// Provides: {"impl_132"}
// Dependencies: {}
impl ICause { # [cfg (target_os = "macos")] fn has_process (self) -> bool { true } # [cfg (not (target_os = "macos"))] fn has_process (self) -> bool { use ICause :: * ; match self { Unknown | Kernel => false , User | TKill | Queue | MesgQ | Exited | Killed | Dumped | Trapped | Stopped | Continued => true , } } }
};
}
