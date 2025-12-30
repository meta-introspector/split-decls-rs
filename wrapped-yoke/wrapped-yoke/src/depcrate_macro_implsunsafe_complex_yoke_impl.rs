// Generated macro for unsafe_complex_yoke_impl (macro)
macro_rules! Depcrate_macro_implsunsafe_complex_yoke_impl {
() => {
// Module: crate::macro_impls
// Provides: {"unsafe_complex_yoke_impl"}
// Dependencies: {}
macro_rules ! unsafe_complex_yoke_impl { () => { fn transform (&'a self) -> &'a Self :: Output { unsafe { mem :: transmute (self) } } fn transform_owned (self) -> Self :: Output { debug_assert ! (mem :: size_of ::< Self :: Output > () == mem :: size_of ::< Self > ()) ; unsafe { let ptr : * const Self :: Output = (& self as * const Self) . cast () ; let _ = ManuallyDrop :: new (self) ; ptr :: read (ptr) } } unsafe fn make (from : Self :: Output) -> Self { debug_assert ! (mem :: size_of ::< Self :: Output > () == mem :: size_of ::< Self > ()) ; let ptr : * const Self = (& from as * const Self :: Output) . cast () ; let _ = ManuallyDrop :: new (from) ; unsafe { ptr :: read (ptr) } } fn transform_mut < F > (&'a mut self , f : F) where F : 'static + for <'b > FnOnce (&'b mut Self :: Output) , { unsafe { f (mem :: transmute ::<&'a mut Self , &'a mut Self :: Output > (self)) } } } ; }
};
}
