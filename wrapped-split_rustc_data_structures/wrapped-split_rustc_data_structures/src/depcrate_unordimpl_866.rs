// Generated macro for impl_866 (impl)
macro_rules! Depcrate_unordimpl_866 {
() => {
// Module: crate::unord
// Provides: {"impl_866"}
// Dependencies: {}
impl < V : Hash + Eq > From < FxHashSet < V > > for UnordSet < V > { fn from (value : FxHashSet < V >) -> Self { UnordSet { inner : value } } }
};
}
