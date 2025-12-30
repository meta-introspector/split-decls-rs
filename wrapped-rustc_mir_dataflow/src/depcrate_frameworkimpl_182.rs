// Generated macro for impl_182 (impl)
macro_rules! Depcrate_frameworkimpl_182 {
() => {
// Module: crate::framework
// Provides: {"impl_182"}
// Dependencies: {}
impl < T , S : GenKill < T > > GenKill < T > for MaybeReachable < S > { fn gen_ (& mut self , elem : T) { match self { MaybeReachable :: Unreachable => { } MaybeReachable :: Reachable (set) => set . gen_ (elem) , } } fn kill (& mut self , elem : T) { match self { MaybeReachable :: Unreachable => { } MaybeReachable :: Reachable (set) => set . kill (elem) , } } }
};
}
