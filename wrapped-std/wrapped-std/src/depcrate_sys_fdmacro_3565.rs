// Generated macro for macro_3565 (macro)
macro_rules! Depcrate_sys_fdmacro_3565 {
() => {
// Module: crate::sys::fd
// Provides: {"macro_3565"}
// Dependencies: {}
cfg_select ! { target_family = "unix" => { mod unix ; pub use unix ::*; } target_os = "hermit" => { mod hermit ; pub use hermit ::*; } all (target_vendor = "fortanix" , target_env = "sgx") => { mod sgx ; pub use sgx ::*; } target_os = "wasi" => { mod wasi ; pub use wasi ::*; } _ => { } }
};
}
