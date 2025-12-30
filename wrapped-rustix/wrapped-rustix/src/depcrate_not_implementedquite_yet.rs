// Generated macro for quite_yet (module)
macro_rules! Depcrate_not_implementedquite_yet {
() => {
// Module: crate::not_implemented
// Provides: {"quite_yet"}
// Dependencies: {}
# [doc = " These functions are not quite yet finished in rustix."] # [doc = ""] # [doc = " Rustix's codebase includes experimental implementations of these functions,"] # [doc = " however they are not yet publicly exposed because their API might need more"] # [doc = " work and/or they don't yet have a libc backend implementation yet."] # [doc = ""] # [doc = " See [#1314] for more information, and please leave comments if there are"] # [doc = " specific functions you're interested in."] # [doc = ""] # [doc = " [#1314]: https://github.com/bytecodealliance/rustix/issues/1314"] pub mod quite_yet { not_implemented ! (_exit) ; not_implemented ! (_Exit) ; not_implemented ! (exit_group) ; not_implemented ! (sigpending) ; not_implemented ! (sigsuspend) ; not_implemented ! (execveat) ; not_implemented ! (execve) ; # [doc = " For now, use `rustix::process::uname().nodename()` instead."] # [doc = ""] # [doc = " See also the [module comment](self)."] pub fn gethostname () { unimplemented ! () } }
};
}
