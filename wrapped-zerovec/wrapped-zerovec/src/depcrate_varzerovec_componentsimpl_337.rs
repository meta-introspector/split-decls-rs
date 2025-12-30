// Generated macro for impl_337 (impl)
macro_rules! Depcrate_varzerovec_componentsimpl_337 {
() => {
// Module: crate::varzerovec::components
// Provides: {"impl_337"}
// Dependencies: {}
impl < 'a , T : VarULE + ? Sized , F : VarZeroVecFormat > Iterator for VarZeroSliceIter < 'a , T , F > { type Item = & 'a T ; fn next (& mut self) -> Option < Self :: Item > { if self . index >= self . components . len () { return None ; } let end = if self . index + 1 == self . components . len () { self . components . things . len () } else { unsafe { self . components . indices_slice () . get_unchecked (self . index) . iule_to_usize () } } ; let item = unsafe { T :: from_bytes_unchecked (self . components . things . get_unchecked (self . start_index .. end)) } ; self . index += 1 ; self . start_index = end ; Some (item) } fn size_hint (& self) -> (usize , Option < usize >) { let remainder = self . components . len () - self . index ; (remainder , Some (remainder)) } }
};
}
