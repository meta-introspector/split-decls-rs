// Generated macro for io_slice (module)
macro_rules! Depcrate_sys_ioio_slice {
() => {
// Module: crate::sys::io
// Provides: {"io_slice"}
// Dependencies: {}
mod io_slice { cfg_select ! { any (target_family = "unix" , target_os = "hermit" , target_os = "solid_asp3" , target_os = "trusty") => { mod iovec ; pub use iovec ::*; } target_os = "windows" => { mod windows ; pub use windows ::*; } target_os = "wasi" => { mod wasi ; pub use wasi ::*; } target_os = "uefi" => { mod uefi ; pub use uefi ::*; } _ => { mod unsupported ; pub use unsupported ::*; } } }
};
}
