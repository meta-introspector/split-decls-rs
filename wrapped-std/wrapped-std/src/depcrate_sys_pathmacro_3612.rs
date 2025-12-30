// Generated macro for macro_3612 (macro)
macro_rules! Depcrate_sys_pathmacro_3612 {
() => {
// Module: crate::sys::path
// Provides: {"macro_3612"}
// Dependencies: {}
cfg_select ! { target_os = "windows" => { mod windows ; mod windows_prefix ; pub use windows ::*; } all (target_vendor = "fortanix" , target_env = "sgx") => { mod sgx ; pub use sgx ::*; } target_os = "solid_asp3" => { mod unsupported_backslash ; pub use unsupported_backslash ::*; } target_os = "uefi" => { mod uefi ; pub use uefi ::*; } target_os = "cygwin" => { mod cygwin ; mod windows_prefix ; pub use cygwin ::*; } _ => { mod unix ; pub use unix ::*; } }
};
}
