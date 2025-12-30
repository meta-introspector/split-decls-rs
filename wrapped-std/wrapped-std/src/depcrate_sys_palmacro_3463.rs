// Generated macro for macro_3463 (macro)
macro_rules! Depcrate_sys_palmacro_3463 {
() => {
// Module: crate::sys::pal
// Provides: {"macro_3463"}
// Dependencies: {}
cfg_select ! { unix => { mod unix ; pub use self :: unix ::*; } windows => { mod windows ; pub use self :: windows ::*; } target_os = "solid_asp3" => { mod solid ; pub use self :: solid ::*; } target_os = "hermit" => { mod hermit ; pub use self :: hermit ::*; } target_os = "trusty" => { mod trusty ; pub use self :: trusty ::*; } all (target_os = "wasi" , target_env = "p2") => { mod wasip2 ; pub use self :: wasip2 ::*; } all (target_os = "wasi" , target_env = "p1") => { mod wasip1 ; pub use self :: wasip1 ::*; } target_family = "wasm" => { mod wasm ; pub use self :: wasm ::*; } target_os = "xous" => { mod xous ; pub use self :: xous ::*; } target_os = "uefi" => { mod uefi ; pub use self :: uefi ::*; } all (target_vendor = "fortanix" , target_env = "sgx") => { mod sgx ; pub use self :: sgx ::*; } target_os = "teeos" => { mod teeos ; pub use self :: teeos ::*; } target_os = "zkvm" => { mod zkvm ; pub use self :: zkvm ::*; } _ => { mod unsupported ; pub use self :: unsupported ::*; } }
};
}
