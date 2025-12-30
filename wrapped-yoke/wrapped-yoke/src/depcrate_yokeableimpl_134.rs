// Generated macro for impl_134 (impl)
macro_rules! Depcrate_yokeableimpl_134 {
() => {
// Module: crate::yokeable
// Provides: {"impl_134"}
// Dependencies: {}
unsafe impl < 'a , T : 'static + ? Sized > Yokeable < 'a > for & 'static T { type Output = & 'a T ; # [inline] fn transform (& 'a self) -> & 'a & 'a T { self } # [inline] fn transform_owned (self) -> & 'a T { self } # [inline] unsafe fn make (from : & 'a T) -> Self { unsafe { mem :: transmute (from) } } # [inline] fn transform_mut < F > (& 'a mut self , f : F) where F : 'static + for < 'b > FnOnce (& 'b mut Self :: Output) , { unsafe { f (mem :: transmute :: < & 'a mut Self , & 'a mut Self :: Output > (self)) } } }
};
}
