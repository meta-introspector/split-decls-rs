use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// The low-level constants.
///
/// Like the signal numbers.
pub mod consts {
    use libc::c_int;
    /// The signal constants.
    ///
    /// Can be mass-imported by `use signal_hook::consts::signal::*`, without polluting the
    /// namespace with other names. Also available in the [`consts`][crate::consts] directly (but
    /// with more constants around).
    pub mod signal {
        #[cfg(any(
            target_os = "freebsd",
            target_os = "dragonfly",
            target_os = "netbsd",
            target_os = "openbsd",
            target_os = "macos"
        ))]
        pub use libc::SIGINFO;
        #[cfg(not(any(windows, target_os = "haiku")))]
        pub use libc::SIGIO;
        #[cfg(not(windows))]
        pub use libc::{
            SIGABRT, SIGALRM, SIGBUS, SIGCHLD, SIGCONT, SIGFPE, SIGHUP, SIGILL, SIGINT, SIGKILL,
            SIGPIPE, SIGPROF, SIGQUIT, SIGSEGV, SIGSTOP, SIGSYS, SIGTERM, SIGTRAP, SIGTSTP,
            SIGTTIN, SIGTTOU, SIGURG, SIGUSR1, SIGUSR2, SIGVTALRM, SIGWINCH, SIGXCPU, SIGXFSZ,
        };
        #[cfg(windows)]
        pub use libc::{SIGABRT, SIGFPE, SIGILL, SIGINT, SIGSEGV, SIGTERM};
        #[cfg(windows)]
        /// Same as `SIGABRT`, but the number is compatible to other platforms.
        pub const SIGABRT_COMPAT: libc::c_int = 6;
        #[cfg(windows)]
        /// Ctrl-Break is pressed for Windows Console processes.
        pub const SIGBREAK: libc::c_int = 21;
    }
    pub use self::signal::*;
    pub use signal_hook_registry::FORBIDDEN;
    /// Various signals commonly requesting shutdown of an application.
    #[cfg(not(windows))]
    pub const TERM_SIGNALS: &[c_int] = &[SIGTERM, SIGQUIT, SIGINT];
    /// Various signals commonly requesting shutdown of an application.
    #[cfg(windows)]
    pub const TERM_SIGNALS: &[c_int] = &[SIGTERM, SIGINT];
}
