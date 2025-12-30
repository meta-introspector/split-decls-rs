// Generated macro for impl_65 (impl)
macro_rules! Depcrate_bagimpl_65 {
() => {
// Module: crate::bag
// Provides: {"impl_65"}
// Dependencies: {}
impl < T , const ARRAY_LEN : usize > Drop for Storage < T , ARRAY_LEN > { # [inline] fn drop (& mut self) { if needs_drop :: < T > () { let mut instance_bitmap = Self :: instance_bitmap (self . metadata . load (Acquire)) ; loop { let index = instance_bitmap . trailing_zeros () ; if index == 32 { break ; } instance_bitmap &= ! (1_u32 << index) ; unsafe { drop_in_place ((* self . storage . get ()) [index as usize] . as_mut_ptr ()) } ; } } } }
};
}
