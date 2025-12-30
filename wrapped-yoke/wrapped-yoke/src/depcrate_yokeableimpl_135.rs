// Generated macro for impl_135 (impl)
macro_rules! Depcrate_yokeableimpl_135 {
() => {
// Module: crate::yokeable
// Provides: {"impl_135"}
// Dependencies: {}
# [cfg (feature = "alloc")] unsafe impl < 'a , T : 'static > Yokeable < 'a > for alloc :: vec :: Vec < T > { type Output = alloc :: vec :: Vec < T > ; # [inline] fn transform (& 'a self) -> & 'a alloc :: vec :: Vec < T > { self } # [inline] fn transform_owned (self) -> alloc :: vec :: Vec < T > { self } # [inline] unsafe fn make (from : alloc :: vec :: Vec < T >) -> Self { from } # [inline] fn transform_mut < F > (& 'a mut self , f : F) where F : 'static + for < 'b > FnOnce (& 'b mut Self :: Output) , { f (self) } }
};
}
