// Generated macro for impl_75 (impl)
macro_rules! Depcrate_iterator_exfiltratorimpl_75 {
() => {
// Module: crate::iterator::exfiltrator
// Provides: {"impl_75"}
// Dependencies: {}
unsafe impl sealed :: Exfiltrator for SignalOnly { type Storage = AtomicBool ; fn supports_signal (& self , _ : c_int) -> bool { true } type Output = c_int ; fn store (& self , slot : & Self :: Storage , _ : c_int , _ : & siginfo_t) { slot . store (true , Ordering :: SeqCst) ; } fn load (& self , slot : & Self :: Storage , signal : c_int) -> Option < Self :: Output > { if slot . compare_exchange (true , false , Ordering :: SeqCst , Ordering :: Relaxed) . is_ok () { Some (signal) } else { None } } }
};
}
