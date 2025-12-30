// Generated macro for guard (module)
macro_rules! Depcrate_sys_thread_localguard {
() => {
// Module: crate::sys::thread_local
// Provides: {"guard"}
// Dependencies: {}
# [doc = " This module provides a way to schedule the execution of the destructor list"] # [doc = " and the [runtime cleanup](crate::rt::thread_cleanup) function. Calling `enable`"] # [doc = " should ensure that these functions are called at the right times."] pub (crate) mod guard { cfg_select ! { all (target_thread_local , target_vendor = "apple") => { mod apple ; pub (crate) use apple :: enable ; } target_os = "windows" => { mod windows ; pub (crate) use windows :: enable ; } any (all (target_family = "wasm" , not (all (target_os = "wasi" , target_env = "p1" , target_feature = "atomics"))) , target_os = "uefi" , target_os = "zkvm" , target_os = "trusty" ,) => { pub (crate) fn enable () { # [cfg (all (target_family = "wasm" , target_feature = "atomics"))] # [allow (unused)] use super :: destructors :: run ; # [allow (unused)] use crate :: rt :: thread_cleanup ; } } any (target_os = "hermit" , target_os = "xous" ,) => { pub (crate) fn enable () { } } target_os = "solid_asp3" => { mod solid ; pub (crate) use solid :: enable ; } _ => { mod key ; pub (crate) use key :: enable ; } } }
};
}
