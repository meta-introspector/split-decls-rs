// Generated macro for impl_833 (impl)
macro_rules! Depcrate_unix_linux_processimpl_833 {
() => {
// Module: crate::unix::linux::process
// Provides: {"impl_833"}
// Dependencies: {}
impl fmt :: Display for ProcessStatus { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . write_str (match * self { ProcessStatus :: Idle => "Idle" , ProcessStatus :: Run => "Runnable" , ProcessStatus :: Sleep => "Sleeping" , ProcessStatus :: Stop => "Stopped" , ProcessStatus :: Zombie => "Zombie" , ProcessStatus :: Tracing => "Tracing" , ProcessStatus :: Dead => "Dead" , ProcessStatus :: Wakekill => "Wakekill" , ProcessStatus :: Waking => "Waking" , ProcessStatus :: Parked => "Parked" , ProcessStatus :: UninterruptibleDiskSleep => "UninterruptibleDiskSleep" , _ => "Unknown" , }) } }
};
}
