// Generated macro for get_current_pid (function)
macro_rules! Depcrate_common_systemget_current_pid {
() => {
// Module: crate::common::system
// Provides: {"get_current_pid"}
// Dependencies: {}
# [doc = " Returns the pid for the current process."] # [doc = ""] # [doc = " `Err` is returned in case the platform isn't supported."] # [doc = ""] # [doc = " ```no_run"] # [doc = " use sysinfo::get_current_pid;"] # [doc = ""] # [doc = " match get_current_pid() {"] # [doc = "     Ok(pid) => {"] # [doc = "         println!(\"current pid: {}\", pid);"] # [doc = "     }"] # [doc = "     Err(e) => {"] # [doc = "         println!(\"failed to get current pid: {}\", e);"] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] # [allow (clippy :: unnecessary_wraps)] pub fn get_current_pid () -> Result < Pid , & 'static str > { cfg_if ! { if # [cfg (feature = "unknown-ci")] { fn inner () -> Result < Pid , &'static str > { Err ("Unknown platform (CI)") } } else if # [cfg (any (target_os = "freebsd" , target_os = "linux" , target_os = "android" , target_os = "macos" , target_os = "ios" ,))] { fn inner () -> Result < Pid , &'static str > { unsafe { Ok (Pid (libc :: getpid ())) } } } else if # [cfg (windows)] { fn inner () -> Result < Pid , &'static str > { use windows :: Win32 :: System :: Threading :: GetCurrentProcessId ; unsafe { Ok (Pid (GetCurrentProcessId () as _)) } } } else { fn inner () -> Result < Pid , &'static str > { Err ("Unknown platform") } } } inner () }
};
}
