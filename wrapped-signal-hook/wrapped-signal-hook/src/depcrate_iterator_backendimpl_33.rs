// Generated macro for impl_33 (impl)
macro_rules! Depcrate_iterator_backendimpl_33 {
() => {
// Module: crate::iterator::backend
// Provides: {"impl_33"}
// Dependencies: {}
impl < E : Exfiltrator > PendingSignals < E > { fn new (exfiltrator : E) -> Self { let mut slots = MaybeUninit :: < [E :: Storage ; MAX_SIGNUM] > :: uninit () ; for i in 0 .. MAX_SIGNUM { unsafe { let slot : * mut E :: Storage = slots . as_mut_ptr () as * mut _ ; let slot = slot . add (i) ; ptr :: write (slot , E :: Storage :: default ()) ; } } Self { exfiltrator , slots : unsafe { slots . assume_init () } , } } }
};
}
