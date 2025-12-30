// Generated macro for destructors (module)
macro_rules! Depcrate_sys_thread_localdestructors {
() => {
// Module: crate::sys::thread_local
// Provides: {"destructors"}
// Dependencies: {}
# [doc = " The native TLS implementation needs a way to register destructors for its data."] # [doc = " This module contains platform-specific implementations of that register."] # [doc = ""] # [doc = " It turns out however that most platforms don't have a way to register a"] # [doc = " destructor for each variable. On these platforms, we keep track of the"] # [doc = " destructors ourselves and register (through the [`guard`] module) only a"] # [doc = " single callback that runs all of the destructors in the list."] # [cfg (all (target_thread_local , not (all (target_family = "wasm" , not (target_feature = "atomics")))))] pub (crate) mod destructors { cfg_select ! { any (target_os = "linux" , target_os = "android" , target_os = "fuchsia" , target_os = "redox" , target_os = "hurd" , target_os = "netbsd" , target_os = "dragonfly") => { mod linux_like ; mod list ; pub (super) use linux_like :: register ; pub (super) use list :: run ; } _ => { mod list ; pub (super) use list :: register ; pub (crate) use list :: run ; } } }
};
}
