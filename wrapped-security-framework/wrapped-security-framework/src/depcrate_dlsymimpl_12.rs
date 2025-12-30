// Generated macro for impl_12 (impl)
macro_rules! Depcrate_dlsymimpl_12 {
() => {
// Module: crate::dlsym
// Provides: {"impl_12"}
// Dependencies: {}
impl < F > DlSym < F > { pub fn get (& self) -> Option < & F > { assert_eq ! (mem :: size_of ::< F > () , mem :: size_of ::< usize > ()) ; unsafe { if self . addr . load (Ordering :: SeqCst) == 0 { self . addr . store (fetch (self . name) , Ordering :: SeqCst) ; } if self . addr . load (Ordering :: SeqCst) == 1 { None } else { mem :: transmute :: < & AtomicUsize , Option < & F > > (& self . addr) } } } }
};
}
