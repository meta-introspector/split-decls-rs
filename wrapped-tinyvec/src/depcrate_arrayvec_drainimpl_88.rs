// Generated macro for impl_88 (impl)
macro_rules! Depcrate_arrayvec_drainimpl_88 {
() => {
// Module: crate::arrayvec_drain
// Provides: {"impl_88"}
// Dependencies: {}
impl < 'a , T : 'a + Default > ArrayVecDrain < 'a , T > { pub (crate) fn new < A , R > (arr : & 'a mut ArrayVec < A > , range : R) -> Self where A : Array < Item = T > , R : RangeBounds < usize > , { let start = match range . start_bound () { Bound :: Unbounded => 0 , Bound :: Included (& n) => n , Bound :: Excluded (& n) => n . saturating_add (1) , } ; let end = match range . end_bound () { Bound :: Unbounded => arr . len () , Bound :: Included (& n) => n . saturating_add (1) , Bound :: Excluded (& n) => n , } ; assert ! (start <= end , "ArrayVec::drain> Illegal range, {} to {}" , start , end) ; assert ! (end <= arr . len () , "ArrayVec::drain> Range ends at {}, but length is only {}" , end , arr . len ()) ; let len = end - start ; let to_rotate = & mut arr [start ..] ; to_rotate . rotate_left (len) ; let oldlen = arr . len () ; let newlen = oldlen - len ; arr . set_len (newlen) ; let slice = & mut arr . data . as_slice_mut () [newlen .. oldlen] ; let iter = slice . iter_mut () ; Self { iter } } }
};
}
