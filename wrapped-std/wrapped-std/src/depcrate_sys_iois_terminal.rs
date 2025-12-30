// Generated macro for is_terminal (module)
macro_rules! Depcrate_sys_iois_terminal {
() => {
// Module: crate::sys::io
// Provides: {"is_terminal"}
// Dependencies: {}
mod is_terminal { cfg_select ! { any (target_family = "unix" , target_os = "wasi") => { mod isatty ; pub use isatty ::*; } target_os = "windows" => { mod windows ; pub use windows ::*; } target_os = "hermit" => { mod hermit ; pub use hermit ::*; } _ => { mod unsupported ; pub use unsupported ::*; } } }
};
}
