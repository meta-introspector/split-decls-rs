// Generated macro for impl_66 (impl)
macro_rules! Depcrate_iterator_exfiltrator_rawimpl_66 {
() => {
// Module: crate::iterator::exfiltrator::raw
// Provides: {"impl_66"}
// Dependencies: {}
unsafe impl Exfiltrator for WithRawSiginfo { type Storage = Slot ; type Output = siginfo_t ; fn supports_signal (& self , _ : c_int) -> bool { true } fn store (& self , slot : & Slot , _ : c_int , info : & siginfo_t) { let info = * info ; if let Some (slot) = unsafe { slot . 0 . load (Ordering :: Acquire) . as_ref () } { slot . send (info) ; } } fn load (& self , slot : & Slot , _ : libc :: c_int) -> Option < siginfo_t > { let slot = unsafe { slot . 0 . load (Ordering :: Acquire) . as_ref () } ; slot . and_then (| s | s . recv ()) } fn init (& self , slot : & Self :: Storage , _ : c_int) { let new = Box :: default () ; let old = slot . 0 . swap (Box :: into_raw (new) , Ordering :: Release) ; assert ! (old . is_null () , "Init called multiple times") ; } }
};
}
