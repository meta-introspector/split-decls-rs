// Generated macro for impl_263 (impl)
macro_rules! Depcrate_ring_buffer_sliceimpl_263 {
() => {
// Module: crate::ring_buffer::slice
// Provides: {"impl_263"}
// Dependencies: {}
impl < 'a , A : PartialEq + 'a , S , const N : usize > PartialEq < S > for Slice < 'a , A , N > where S : Borrow < [A] > , { # [inline] # [must_use] fn eq (& self , other : & S) -> bool { let other = other . borrow () ; self . len () == other . len () && self . iter () . eq (other . iter ()) } }
};
}
