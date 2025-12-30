// Generated macro for impl_659 (impl)
macro_rules! Depcrate_unix_freebsd_processimpl_659 {
() => {
// Module: crate::unix::freebsd::process
// Provides: {"impl_659"}
// Dependencies: {}
impl fmt :: Display for ProcessStatus { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . write_str (match * self { ProcessStatus :: Idle => "Idle" , ProcessStatus :: Run => "Runnable" , ProcessStatus :: Sleep => "Sleeping" , ProcessStatus :: Stop => "Stopped" , ProcessStatus :: Zombie => "Zombie" , ProcessStatus :: Dead => "Dead" , ProcessStatus :: LockBlocked => "LockBlocked" , _ => "Unknown" , }) } }
};
}
