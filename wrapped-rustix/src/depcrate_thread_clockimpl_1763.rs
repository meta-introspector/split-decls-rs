// Generated macro for impl_1763 (impl)
macro_rules! Depcrate_thread_clockimpl_1763 {
() => {
// Module: crate::thread::clock
// Provides: {"impl_1763"}
// Dependencies: {}
impl fmt :: Debug for NanosleepRelativeResult { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Self :: Ok => f . write_str ("Ok") , Self :: Interrupted (remaining) => write ! (f , "Interrupted(Timespec {{ tv_sec: {:?}, tv_nsec: {:?} }})" , remaining . tv_sec , remaining . tv_nsec) , Self :: Err (err) => write ! (f , "Err({:?})" , err) , } } }
};
}
