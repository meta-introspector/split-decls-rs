// Generated macro for macro_3606 (macro)
macro_rules! Depcrate_sys_net_connectionmacro_3606 {
() => {
// Module: crate::sys::net::connection
// Provides: {"macro_3606"}
// Dependencies: {}
cfg_select ! { any (all (target_family = "unix" , not (target_os = "l4re")) , target_os = "windows" , target_os = "hermit" , all (target_os = "wasi" , target_env = "p2") , target_os = "solid_asp3" ,) => { mod socket ; pub use socket ::*; } all (target_vendor = "fortanix" , target_env = "sgx") => { mod sgx ; pub use sgx ::*; } all (target_os = "wasi" , target_env = "p1") => { mod wasip1 ; pub use wasip1 ::*; } target_os = "xous" => { mod xous ; pub use xous ::*; } target_os = "uefi" => { mod uefi ; pub use uefi ::*; } _ => { mod unsupported ; pub use unsupported ::*; } }
};
}
