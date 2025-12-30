// Generated macro for impl_331 (impl)
macro_rules! Depcrate_internedimpl_331 {
() => {
// Module: crate::interned
// Provides: {"impl_331"}
// Dependencies: {}
impl < C : Configuration > Default for RevisionQueue < C > { fn default () -> RevisionQueue < C > { let revisions = if C :: REVISIONS == IMMORTAL { Box :: default () } else { (0 .. C :: REVISIONS . get ()) . map (| _ | AtomicRevision :: start ()) . collect () } ; RevisionQueue { lock : Mutex :: new (()) , revisions , _configuration : PhantomData , } } }
};
}
