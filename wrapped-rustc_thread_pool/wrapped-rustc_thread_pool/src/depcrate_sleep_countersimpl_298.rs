// Generated macro for impl_298 (impl)
macro_rules! Depcrate_sleep_countersimpl_298 {
() => {
// Module: crate::sleep::counters
// Provides: {"impl_298"}
// Dependencies: {}
impl std :: fmt :: Debug for Counters { fn fmt (& self , fmt : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { let word = format ! ("{:016x}" , self . word) ; fmt . debug_struct ("Counters") . field ("word" , & word) . field ("jobs" , & self . jobs_counter () . 0) . field ("inactive" , & self . inactive_threads ()) . field ("sleeping" , & self . sleeping_threads ()) . finish () } }
};
}
