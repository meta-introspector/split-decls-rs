// Generated macro for impl_1319 (impl)
macro_rules! Depcrate_netimpl_1319 {
() => {
// Module: crate::net
// Provides: {"impl_1319"}
// Dependencies: {}
impl fmt :: Display for SocketAddr { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { SocketAddr :: Net (addr) => write ! (f , "{}" , addr) , # [cfg (unix)] SocketAddr :: Unix (p) => write ! (f , "{}" , p . display ()) , # [cfg (any (target_os = "linux" , target_os = "android"))] SocketAddr :: UnixAbstract (p) => write ! (f , "\\x00{}" , p . escape_ascii ()) , } } }
};
}
