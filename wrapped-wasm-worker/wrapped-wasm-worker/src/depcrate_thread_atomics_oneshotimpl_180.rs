// Generated macro for impl_180 (impl)
macro_rules! Depcrate_thread_atomics_oneshotimpl_180 {
() => {
// Module: crate::thread::atomics::oneshot
// Provides: {"impl_180"}
// Dependencies: {}
impl < T > State < T > { # [doc = " Takes the current state if there is one."] fn take (& mut self) -> Option < Self > { match self { Self :: Waiting => None , Self :: Dropped | Self :: Result (_) => Some (mem :: replace (self , Self :: Taken)) , Self :: Taken => Some (Self :: Taken) , } } }
};
}
