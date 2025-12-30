// Generated macro for macro_2 (macro)
macro_rules! Depcratemacro_2 {
() => {
// Module: crate
// Provides: {"macro_2"}
// Dependencies: {}
cfg_select ! { target_env = "msvc" => { } any (target_os = "l4re" , target_os = "none" , target_os = "espidf" , target_os = "nuttx" ,) => { } any (unix , windows , target_os = "psp" , target_os = "solid_asp3" , all (target_vendor = "fortanix" , target_env = "sgx") ,) => { mod libunwind ; pub use libunwind ::*; } target_os = "xous" => { mod unwinding ; pub use unwinding ::*; } target_family = "wasm" => { mod wasm ; pub use wasm ::*; } _ => { } }
};
}
