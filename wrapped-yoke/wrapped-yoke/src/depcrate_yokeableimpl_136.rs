// Generated macro for impl_136 (impl)
macro_rules! Depcrate_yokeableimpl_136 {
() => {
// Module: crate::yokeable
// Provides: {"impl_136"}
// Dependencies: {}
unsafe impl < 'a , T : ? Sized + 'static > Yokeable < 'a > for PhantomData < T > { type Output = PhantomData < T > ; fn transform (& 'a self) -> & 'a Self :: Output { self } fn transform_owned (self) -> Self :: Output { self } unsafe fn make (from : Self :: Output) -> Self { from } fn transform_mut < F > (& 'a mut self , f : F) where F : 'static + for < 'b > FnOnce (& 'b mut Self :: Output) , { f (self) } }
};
}
