// Generated macro for impl_54 (impl)
macro_rules! Depcrate_bagimpl_54 {
() => {
// Module: crate::bag
// Provides: {"impl_54"}
// Dependencies: {}
impl < T , const ARRAY_LEN : usize > Drop for Bag < T , ARRAY_LEN > { # [inline] fn drop (& mut self) { if needs_drop :: < T > () { while let Some (v) = self . pop () { drop (v) ; } } } }
};
}
