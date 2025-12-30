// Generated macro for copy_yoke_impl (macro)
macro_rules! Depcrate_macro_implscopy_yoke_impl {
() => {
// Module: crate::macro_impls
// Provides: {"copy_yoke_impl"}
// Dependencies: {}
macro_rules ! copy_yoke_impl { () => { # [inline] fn transform (& self) -> & Self :: Output { self } # [inline] fn transform_owned (self) -> Self :: Output { self } # [inline] unsafe fn make (this : Self :: Output) -> Self { this } # [inline] fn transform_mut < F > (&'a mut self , f : F) where F : 'static + for <'b > FnOnce (&'b mut Self :: Output) , { f (self) } } ; }
};
}
