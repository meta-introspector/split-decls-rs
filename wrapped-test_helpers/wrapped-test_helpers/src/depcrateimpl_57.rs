// Generated macro for impl_57 (impl)
macro_rules! Depcrateimpl_57 {
() => {
// Module: crate
// Provides: {"impl_57"}
// Dependencies: {}
impl < T > DefaultStrategy for * const T { type Strategy = proptest :: strategy :: Map < proptest :: num :: isize :: Any , fn (isize) -> * const T > ; fn default_strategy () -> Self :: Strategy { fn map < T > (x : isize) -> * const T { x as _ } use proptest :: strategy :: Strategy ; proptest :: num :: isize :: ANY . prop_map (map) } }
};
}
