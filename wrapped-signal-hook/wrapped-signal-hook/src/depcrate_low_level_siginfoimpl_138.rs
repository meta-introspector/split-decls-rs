// Generated macro for impl_138 (impl)
macro_rules! Depcrate_low_level_siginfoimpl_138 {
() => {
// Module: crate::low_level::siginfo
// Provides: {"impl_138"}
// Dependencies: {}
impl From < ICause > for Cause { fn from (c : ICause) -> Cause { match c { ICause :: Kernel => Cause :: Kernel , ICause :: User => Cause :: Sent (Sent :: User) , ICause :: TKill => Cause :: Sent (Sent :: TKill) , ICause :: Queue => Cause :: Sent (Sent :: Queue) , ICause :: MesgQ => Cause :: Sent (Sent :: MesgQ) , ICause :: Exited => Cause :: Chld (Chld :: Exited) , ICause :: Killed => Cause :: Chld (Chld :: Killed) , ICause :: Dumped => Cause :: Chld (Chld :: Dumped) , ICause :: Trapped => Cause :: Chld (Chld :: Trapped) , ICause :: Stopped => Cause :: Chld (Chld :: Stopped) , ICause :: Continued => Cause :: Chld (Chld :: Continued) , _ => Cause :: Unknown , } } }
};
}
