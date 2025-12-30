// Generated macro for impl_70 (impl)
macro_rules! Depcrateimpl_70 {
() => {
// Module: crate
// Provides: {"impl_70"}
// Dependencies: {}
impl < T > DefaultStrategy for * const T { type Strategy = proptest :: strategy :: Map < proptest :: num :: isize :: Any , fn (isize) -> * const T > ; fn default_strategy () -> Self :: Strategy { fn map < T > (x : isize) -> * const T { x as _ } use proptest :: strategy :: Strategy ; proptest :: num :: isize :: ANY . prop_map (map) } }
};
}
