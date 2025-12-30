// Generated macro for impl_38 (impl)
macro_rules! Depcrate_arrayvecimpl_38 {
() => {
// Module: crate::arrayvec
// Provides: {"impl_38"}
// Dependencies: {}
impl < 'p , A : Array , I : Iterator < Item = A :: Item > > Drop for ArrayVecSplice < 'p , A , I > { # [inline] fn drop (& mut self) { for _ in self . by_ref () { } for replacement in self . replacement . by_ref () { self . parent . insert (self . removal_end , replacement) ; self . removal_end += 1 ; } } }
};
}
