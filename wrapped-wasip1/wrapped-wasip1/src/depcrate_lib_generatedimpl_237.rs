// Generated macro for impl_237 (impl)
macro_rules! Depcrate_lib_generatedimpl_237 {
() => {
// Module: crate::lib_generated
// Provides: {"impl_237"}
// Dependencies: {}
impl Signal { pub const fn raw (& self) -> u8 { self . 0 } pub fn name (& self) -> & 'static str { match self . 0 { 0 => "NONE" , 1 => "HUP" , 2 => "INT" , 3 => "QUIT" , 4 => "ILL" , 5 => "TRAP" , 6 => "ABRT" , 7 => "BUS" , 8 => "FPE" , 9 => "KILL" , 10 => "USR1" , 11 => "SEGV" , 12 => "USR2" , 13 => "PIPE" , 14 => "ALRM" , 15 => "TERM" , 16 => "CHLD" , 17 => "CONT" , 18 => "STOP" , 19 => "TSTP" , 20 => "TTIN" , 21 => "TTOU" , 22 => "URG" , 23 => "XCPU" , 24 => "XFSZ" , 25 => "VTALRM" , 26 => "PROF" , 27 => "WINCH" , 28 => "POLL" , 29 => "PWR" , 30 => "SYS" , _ => unsafe { core :: hint :: unreachable_unchecked () } , } } pub fn message (& self) -> & 'static str { match self . 0 { 0 => { "No signal. Note that POSIX has special semantics for `kill(pid, 0)`,
so this value is reserved." } 1 => { "Hangup.
Action: Terminates the process." } 2 => { "Terminate interrupt signal.
Action: Terminates the process." } 3 => { "Terminal quit signal.
Action: Terminates the process." } 4 => { "Illegal instruction.
Action: Terminates the process." } 5 => { "Trace/breakpoint trap.
Action: Terminates the process." } 6 => { "Process abort signal.
Action: Terminates the process." } 7 => { "Access to an undefined portion of a memory object.
Action: Terminates the process." } 8 => { "Erroneous arithmetic operation.
Action: Terminates the process." } 9 => { "Kill.
Action: Terminates the process." } 10 => { "User-defined signal 1.
Action: Terminates the process." } 11 => { "Invalid memory reference.
Action: Terminates the process." } 12 => { "User-defined signal 2.
Action: Terminates the process." } 13 => { "Write on a pipe with no one to read it.
Action: Ignored." } 14 => { "Alarm clock.
Action: Terminates the process." } 15 => { "Termination signal.
Action: Terminates the process." } 16 => { "Child process terminated, stopped, or continued.
Action: Ignored." } 17 => { "Continue executing, if stopped.
Action: Continues executing, if stopped." } 18 => { "Stop executing.
Action: Stops executing." } 19 => { "Terminal stop signal.
Action: Stops executing." } 20 => { "Background process attempting read.
Action: Stops executing." } 21 => { "Background process attempting write.
Action: Stops executing." } 22 => { "High bandwidth data is available at a socket.
Action: Ignored." } 23 => { "CPU time limit exceeded.
Action: Terminates the process." } 24 => { "File size limit exceeded.
Action: Terminates the process." } 25 => { "Virtual timer expired.
Action: Terminates the process." } 26 => { "Profiling timer expired.
Action: Terminates the process." } 27 => { "Window changed.
Action: Ignored." } 28 => { "I/O possible.
Action: Terminates the process." } 29 => { "Power failure.
Action: Terminates the process." } 30 => { "Bad system call.
Action: Terminates the process." } _ => unsafe { core :: hint :: unreachable_unchecked () } , } } }
};
}
