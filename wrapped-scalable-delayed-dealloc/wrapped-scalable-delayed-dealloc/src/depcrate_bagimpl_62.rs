// Generated macro for impl_62 (impl)
macro_rules! Depcrate_bagimpl_62 {
() => {
// Module: crate::bag
// Provides: {"impl_62"}
// Dependencies: {}
impl < T , const ARRAY_LEN : usize > Iterator for IntoIter < T , ARRAY_LEN > { type Item = T ; # [inline] fn next (& mut self) -> Option < Self :: Item > { self . bag . pop () } }
};
}
