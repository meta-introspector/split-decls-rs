// Generated macro for macro_3558 (macro)
macro_rules! Depcrate_sys_envmacro_3558 {
() => {
// Module: crate::sys::env
// Provides: {"macro_3558"}
// Dependencies: {}
cfg_select ! { target_family = "unix" => { mod unix ; pub use unix ::*; } target_family = "windows" => { mod windows ; pub use windows ::*; } target_os = "hermit" => { mod hermit ; pub use hermit ::*; } all (target_vendor = "fortanix" , target_env = "sgx") => { mod sgx ; pub use sgx ::*; } target_os = "solid_asp3" => { mod solid ; pub use solid ::*; } target_os = "uefi" => { mod uefi ; pub use uefi ::*; } target_os = "wasi" => { mod wasi ; pub use wasi ::*; } target_os = "xous" => { mod xous ; pub use xous ::*; } target_os = "zkvm" => { mod zkvm ; pub use zkvm ::*; } _ => { mod unsupported ; pub use unsupported ::*; } }
};
}
