// Generated macro for impl_63 (impl)
macro_rules! Depcrate_jobimpl_63 {
() => {
// Module: crate::job
// Provides: {"impl_63"}
// Dependencies: {}
impl JobRef { # [doc = " Unsafe: caller asserts that `data` will remain valid until the"] # [doc = " job is executed."] pub (super) unsafe fn new < T > (data : * const T) -> JobRef where T : Job , { JobRef { pointer : data as * const () , execute_fn : < T as Job > :: execute } } # [inline] pub (super) fn id (& self) -> JobRefId { JobRefId { pointer : self . pointer . expose_provenance () } } # [inline] pub (super) unsafe fn execute (self) { unsafe { (self . execute_fn) (self . pointer) } } }
};
}
