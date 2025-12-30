// Generated macro for impl_1954 (impl)
macro_rules! Depcrate_thread_schedimpl_1954 {
() => {
// Module: crate::thread::sched
// Provides: {"impl_1954"}
// Dependencies: {}
impl fmt :: Debug for CpuSet { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "CpuSet {{") ? ; let mut first = true ; for i in 0 .. Self :: MAX_CPU { if self . is_set (i) { if first { write ! (f , " ") ? ; first = false ; } else { write ! (f , ", ") ? ; } write ! (f , "cpu{}" , i) ? ; } } write ! (f , " }}") } }
};
}
