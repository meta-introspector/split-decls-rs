// Generated macro for macro_3524 (macro)
macro_rules! Depcrate_sys_argsmacro_3524 {
() => {
// Module: crate::sys::args
// Provides: {"macro_3524"}
// Dependencies: {}
cfg_select ! { any (all (target_family = "unix" , not (any (target_os = "espidf" , target_os = "vita"))) , target_os = "hermit" ,) => { mod unix ; pub use unix ::*; } target_family = "windows" => { mod windows ; pub use windows ::*; } all (target_vendor = "fortanix" , target_env = "sgx") => { mod sgx ; pub use sgx ::*; } target_os = "uefi" => { mod uefi ; pub use uefi ::*; } all (target_os = "wasi" , target_env = "p1") => { mod wasip1 ; pub use wasip1 ::*; } all (target_os = "wasi" , target_env = "p2") => { mod wasip2 ; pub use wasip2 ::*; } target_os = "xous" => { mod xous ; pub use xous ::*; } target_os = "zkvm" => { mod zkvm ; pub use zkvm ::*; } _ => { mod unsupported ; pub use unsupported ::*; } }
};
}
