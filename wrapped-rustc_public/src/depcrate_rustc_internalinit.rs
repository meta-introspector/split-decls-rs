// Generated macro for init (function)
macro_rules! Depcrate_rustc_internalinit {
() => {
// Module: crate::rustc_internal
// Provides: {"init"}
// Dependencies: {}
pub (crate) fn init < 'tcx , F , T , B : Bridge > (container : & Container < 'tcx , B > , f : F) -> T where F : FnOnce () -> T , { assert ! (! TLV . is_set ()) ; let ptr = container as * const _ as * const () ; TLV . set (& Cell :: new (ptr) , | | f ()) }
};
}
