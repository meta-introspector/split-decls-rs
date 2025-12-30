// Generated macro for impl_68 (impl)
macro_rules! Depcrate_cancelledimpl_68 {
() => {
// Module: crate::cancelled
// Provides: {"impl_68"}
// Dependencies: {}
impl Cancelled { # [cold] pub (crate) fn throw (self) -> ! { panic :: resume_unwind (Box :: new (self)) ; } # [doc = " Runs `f`, and catches any salsa cancellation."] pub fn catch < F , T > (f : F) -> Result < T , Cancelled > where F : FnOnce () -> T + UnwindSafe , { match panic :: catch_unwind (f) { Ok (t) => Ok (t) , Err (payload) => match payload . downcast () { Ok (cancelled) => Err (* cancelled) , Err (payload) => panic :: resume_unwind (payload) , } , } } }
};
}
