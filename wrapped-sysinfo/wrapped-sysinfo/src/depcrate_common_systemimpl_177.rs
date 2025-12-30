// Generated macro for impl_177 (impl)
macro_rules! Depcrate_common_systemimpl_177 {
() => {
// Module: crate::common::system
// Provides: {"impl_177"}
// Dependencies: {}
impl std :: fmt :: Display for Signal { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { let s = match * self { Self :: Hangup => "Hangup" , Self :: Interrupt => "Interrupt" , Self :: Quit => "Quit" , Self :: Illegal => "Illegal" , Self :: Trap => "Trap" , Self :: Abort => "Abort" , Self :: IOT => "IOT" , Self :: Bus => "Bus" , Self :: FloatingPointException => "FloatingPointException" , Self :: Kill => "Kill" , Self :: User1 => "User1" , Self :: Segv => "Segv" , Self :: User2 => "User2" , Self :: Pipe => "Pipe" , Self :: Alarm => "Alarm" , Self :: Term => "Term" , Self :: Child => "Child" , Self :: Continue => "Continue" , Self :: Stop => "Stop" , Self :: TSTP => "TSTP" , Self :: TTIN => "TTIN" , Self :: TTOU => "TTOU" , Self :: Urgent => "Urgent" , Self :: XCPU => "XCPU" , Self :: XFSZ => "XFSZ" , Self :: VirtualAlarm => "VirtualAlarm" , Self :: Profiling => "Profiling" , Self :: Winch => "Winch" , Self :: IO => "IO" , Self :: Poll => "Poll" , Self :: Power => "Power" , Self :: Sys => "Sys" , } ; f . write_str (s) } }
};
}
