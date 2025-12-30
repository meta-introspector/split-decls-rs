// Generated macro for impl_216 (impl)
macro_rules! Depcrate_unique_arcimpl_216 {
() => {
// Module: crate::unique_arc
// Provides: {"impl_216"}
// Dependencies: {}
impl < T : ? Sized > TryFrom < Arc < T > > for UniqueArc < T > { type Error = Arc < T > ; fn try_from (arc : Arc < T >) -> Result < Self , Self :: Error > { Arc :: try_unique (arc) } }
};
}
