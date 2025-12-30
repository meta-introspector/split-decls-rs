// Generated macro for macro_3471 (macro)
macro_rules! Depcrate_sys_allocmacro_3471 {
() => {
// Module: crate::sys::alloc
// Provides: {"macro_3471"}
// Dependencies: {}
cfg_select ! { any (target_family = "unix" , target_os = "wasi" , target_os = "teeos" , target_os = "trusty" ,) => { mod unix ; } target_os = "windows" => { mod windows ; } target_os = "hermit" => { mod hermit ; } all (target_vendor = "fortanix" , target_env = "sgx") => { mod sgx ; } target_os = "solid_asp3" => { mod solid ; } target_os = "uefi" => { mod uefi ; } target_family = "wasm" => { mod wasm ; } target_os = "xous" => { mod xous ; } target_os = "zkvm" => { mod zkvm ; } }
};
}
