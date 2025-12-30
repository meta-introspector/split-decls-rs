// Generated macro for macro_3680 (macro)
macro_rules! Depcrate_sys_processmacro_3680 {
() => {
// Module: crate::sys::process
// Provides: {"macro_3680"}
// Dependencies: {}
cfg_select ! { target_family = "unix" => { mod unix ; use unix as imp ; } target_os = "windows" => { mod windows ; use windows as imp ; } target_os = "uefi" => { mod uefi ; use uefi as imp ; } _ => { mod unsupported ; use unsupported as imp ; } }
};
}
