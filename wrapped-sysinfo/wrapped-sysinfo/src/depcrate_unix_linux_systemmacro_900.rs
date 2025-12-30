// Generated macro for macro_900 (macro)
macro_rules! Depcrate_unix_linux_systemmacro_900 {
() => {
// Module: crate::unix::linux::system
// Provides: {"macro_900"}
// Dependencies: {}
declare_signals ! { libc :: c_int , Signal :: Hangup => libc :: SIGHUP , Signal :: Interrupt => libc :: SIGINT , Signal :: Quit => libc :: SIGQUIT , Signal :: Illegal => libc :: SIGILL , Signal :: Trap => libc :: SIGTRAP , Signal :: Abort => libc :: SIGABRT , Signal :: IOT => libc :: SIGIOT , Signal :: Bus => libc :: SIGBUS , Signal :: FloatingPointException => libc :: SIGFPE , Signal :: Kill => libc :: SIGKILL , Signal :: User1 => libc :: SIGUSR1 , Signal :: Segv => libc :: SIGSEGV , Signal :: User2 => libc :: SIGUSR2 , Signal :: Pipe => libc :: SIGPIPE , Signal :: Alarm => libc :: SIGALRM , Signal :: Term => libc :: SIGTERM , Signal :: Child => libc :: SIGCHLD , Signal :: Continue => libc :: SIGCONT , Signal :: Stop => libc :: SIGSTOP , Signal :: TSTP => libc :: SIGTSTP , Signal :: TTIN => libc :: SIGTTIN , Signal :: TTOU => libc :: SIGTTOU , Signal :: Urgent => libc :: SIGURG , Signal :: XCPU => libc :: SIGXCPU , Signal :: XFSZ => libc :: SIGXFSZ , Signal :: VirtualAlarm => libc :: SIGVTALRM , Signal :: Profiling => libc :: SIGPROF , Signal :: Winch => libc :: SIGWINCH , Signal :: IO => libc :: SIGIO , Signal :: Poll => libc :: SIGPOLL , Signal :: Power => libc :: SIGPWR , Signal :: Sys => libc :: SIGSYS , }
};
}
