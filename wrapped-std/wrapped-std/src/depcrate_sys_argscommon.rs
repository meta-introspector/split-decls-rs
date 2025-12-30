// Generated macro for common (module)
macro_rules! Depcrate_sys_argscommon {
() => {
// Module: crate::sys::args
// Provides: {"common"}
// Dependencies: {}
# [cfg (any (all (target_family = "unix" , not (any (target_os = "espidf" , target_os = "vita"))) , target_family = "windows" , target_os = "hermit" , target_os = "uefi" , target_os = "wasi" , target_os = "xous" ,))] mod common ;
};
}
