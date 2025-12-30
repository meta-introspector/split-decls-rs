// Generated macro for RtlUserProcessParameters (trait)
macro_rules! Depcrate_windows_processRtlUserProcessParameters {
() => {
// Module: crate::windows::process
// Provides: {"RtlUserProcessParameters"}
// Dependencies: {}
trait RtlUserProcessParameters { fn get_cmdline (& self , handle : HANDLE) -> Result < Vec < u16 > , & 'static str > ; fn get_cwd (& self , handle : HANDLE) -> Result < Vec < u16 > , & 'static str > ; fn get_environ (& self , handle : HANDLE) -> Result < Vec < u16 > , & 'static str > ; }
};
}
