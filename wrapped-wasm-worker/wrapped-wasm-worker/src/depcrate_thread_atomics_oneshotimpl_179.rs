// Generated macro for impl_179 (impl)
macro_rules! Depcrate_thread_atomics_oneshotimpl_179 {
() => {
// Module: crate::thread::atomics::oneshot
// Provides: {"impl_179"}
// Dependencies: {}
impl < T > Debug for Shared < T > { fn fmt (& self , formatter : & mut Formatter < '_ >) -> fmt :: Result { formatter . debug_struct ("Shared") . field ("value" , & any :: type_name_of_val (& self . value)) . field ("cvar" , & self . cvar) . field ("waker" , & self . waker) . finish () } }
};
}
