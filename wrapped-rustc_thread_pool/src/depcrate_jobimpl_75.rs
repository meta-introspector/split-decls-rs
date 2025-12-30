// Generated macro for impl_75 (impl)
macro_rules! Depcrate_jobimpl_75 {
() => {
// Module: crate::job
// Provides: {"impl_75"}
// Dependencies: {}
impl JobFifo { pub (super) fn new () -> Self { JobFifo { inner : Injector :: new () } } pub (super) unsafe fn push (& self , job_ref : JobRef) -> JobRef { self . inner . push (job_ref) ; unsafe { JobRef :: new (self) } } }
};
}
