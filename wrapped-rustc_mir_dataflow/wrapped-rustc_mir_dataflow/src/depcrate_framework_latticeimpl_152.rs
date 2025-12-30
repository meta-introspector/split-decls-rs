// Generated macro for impl_152 (impl)
macro_rules! Depcrate_framework_latticeimpl_152 {
() => {
// Module: crate::framework::lattice
// Provides: {"impl_152"}
// Dependencies: {}
impl < S > MaybeReachable < S > { # [doc = " Return whether the current state contains the given element. If the state is unreachable,"] # [doc = " it does no contain anything."] pub fn contains < T > (& self , elem : T) -> bool where S : BitSetExt < T > , { match self { MaybeReachable :: Unreachable => false , MaybeReachable :: Reachable (set) => set . contains (elem) , } } }
};
}
