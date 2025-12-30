// Generated macro for DeferredClosure (struct)
macro_rules! Depcrate_collectibleDeferredClosure {
() => {
// Module: crate::collectible
// Provides: {"DeferredClosure"}
// Dependencies: {}
# [doc = " [`DeferredClosure`] implements [`Collectible`] for a closure to execute it after all the"] # [doc = " current readers in the process are gone."] pub (super) struct DeferredClosure < F : 'static + FnOnce () > { f : Option < F > , link : Link , }
};
}
