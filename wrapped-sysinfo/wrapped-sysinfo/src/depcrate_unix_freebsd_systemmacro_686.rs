// Generated macro for macro_686 (macro)
macro_rules! Depcrate_unix_freebsd_systemmacro_686 {
() => {
// Module: crate::unix::freebsd::system
// Provides: {"macro_686"}
// Dependencies: {}
declare_signals ! { c_int , Signal :: Hangup => libc :: SIGHUP , Signal :: Interrupt => libc :: SIGINT , Signal :: Quit => libc :: SIGQUIT , Signal :: Illegal => libc :: SIGILL , Signal :: Trap => libc :: SIGTRAP , Signal :: Abort => libc :: SIGABRT , Signal :: IOT => libc :: SIGIOT , Signal :: Bus => libc :: SIGBUS , Signal :: FloatingPointException => libc :: SIGFPE , Signal :: Kill => libc :: SIGKILL , Signal :: User1 => libc :: SIGUSR1 , Signal :: Segv => libc :: SIGSEGV , Signal :: User2 => libc :: SIGUSR2 , Signal :: Pipe => libc :: SIGPIPE , Signal :: Alarm => libc :: SIGALRM , Signal :: Term => libc :: SIGTERM , Signal :: Child => libc :: SIGCHLD , Signal :: Continue => libc :: SIGCONT , Signal :: Stop => libc :: SIGSTOP , Signal :: TSTP => libc :: SIGTSTP , Signal :: TTIN => libc :: SIGTTIN , Signal :: TTOU => libc :: SIGTTOU , Signal :: Urgent => libc :: SIGURG , Signal :: XCPU => libc :: SIGXCPU , Signal :: XFSZ => libc :: SIGXFSZ , Signal :: VirtualAlarm => libc :: SIGVTALRM , Signal :: Profiling => libc :: SIGPROF , Signal :: Winch => libc :: SIGWINCH , Signal :: IO => libc :: SIGIO , Signal :: Sys => libc :: SIGSYS , _ => None , }
};
}
