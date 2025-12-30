// Generated macro for impl_516 (impl)
macro_rules! Depcrate_unix_apple_processimpl_516 {
() => {
// Module: crate::unix::apple::process
// Provides: {"impl_516"}
// Dependencies: {}
impl fmt :: Display for ProcessStatus { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . write_str (match * self { ProcessStatus :: Idle => "Idle" , ProcessStatus :: Run => "Runnable" , ProcessStatus :: Sleep => "Sleeping" , ProcessStatus :: Stop => "Stopped" , ProcessStatus :: Zombie => "Zombie" , _ => "Unknown" , }) } }
};
}
