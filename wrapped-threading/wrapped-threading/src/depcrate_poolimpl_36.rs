// Generated macro for impl_36 (impl)
macro_rules! Depcrate_poolimpl_36 {
() => {
// Module: crate::pool
// Provides: {"impl_36"}
// Dependencies: {}
impl < 'scope , 'env > Scope < 'scope , 'env > { # [doc = " Submits the closure to run on the `Pool`."] # [doc = ""] # [doc = " The closure cannot outlive the `Scope` it's run in."] pub fn submit < F : FnOnce () + Send + 'scope > (& 'scope self , f : F) { unsafe { try_submit (& * self . pool . 0 , f) ; } } }
};
}
