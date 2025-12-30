// Generated macro for impl_58 (impl)
macro_rules! Depcrateimpl_58 {
() => {
// Module: crate
// Provides: {"impl_58"}
// Dependencies: {}
impl < T > DefaultStrategy for * mut T { type Strategy = proptest :: strategy :: Map < proptest :: num :: isize :: Any , fn (isize) -> * mut T > ; fn default_strategy () -> Self :: Strategy { fn map < T > (x : isize) -> * mut T { x as _ } use proptest :: strategy :: Strategy ; proptest :: num :: isize :: ANY . prop_map (map) } }
};
}
