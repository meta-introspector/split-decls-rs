// Generated macro for impl_213 (impl)
macro_rules! Depcrate_collectibleimpl_213 {
() => {
// Module: crate::collectible
// Provides: {"impl_213"}
// Dependencies: {}
impl < F : 'static + FnOnce () > DeferredClosure < F > { # [doc = " Creates a new [`DeferredClosure`]."] # [inline] pub fn new (f : F) -> Self { DeferredClosure { f : Some (f) , link : Link :: default () , } } }
};
}
