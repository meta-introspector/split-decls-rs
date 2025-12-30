// Generated macro for impl_578 (impl)
macro_rules! Depcrate_ule_charsimpl_578 {
() => {
// Module: crate::ule::chars
// Provides: {"impl_578"}
// Dependencies: {}
impl Ord for CharULE { fn cmp (& self , other : & Self) -> Ordering { char :: from_unaligned (* self) . cmp (& char :: from_unaligned (* other)) } }
};
}
