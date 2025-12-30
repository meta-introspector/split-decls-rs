// Generated macro for impl_71 (impl)
macro_rules! Depcrateimpl_71 {
() => {
// Module: crate
// Provides: {"impl_71"}
// Dependencies: {}
impl < T > DefaultStrategy for * mut T { type Strategy = proptest :: strategy :: Map < proptest :: num :: isize :: Any , fn (isize) -> * mut T > ; fn default_strategy () -> Self :: Strategy { fn map < T > (x : isize) -> * mut T { x as _ } use proptest :: strategy :: Strategy ; proptest :: num :: isize :: ANY . prop_map (map) } }
};
}
