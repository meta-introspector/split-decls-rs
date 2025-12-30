// Generated macro for impl_RtlUserProcessParameters (macro)
macro_rules! Depcrate_windows_processimpl_RtlUserProcessParameters {
() => {
// Module: crate::windows::process
// Provides: {"impl_RtlUserProcessParameters"}
// Dependencies: {}
macro_rules ! impl_RtlUserProcessParameters { ($ t : ty) => { impl RtlUserProcessParameters for $ t { fn get_cmdline (& self , handle : HANDLE) -> Result < Vec < u16 >, &'static str > { let ptr = self . CommandLine . Buffer ; let size = self . CommandLine . Length ; unsafe { get_process_data (handle , ptr as _ , size as _) } } fn get_cwd (& self , handle : HANDLE) -> Result < Vec < u16 >, &'static str > { let ptr = self . CurrentDirectory . DosPath . Buffer ; let size = self . CurrentDirectory . DosPath . Length ; unsafe { get_process_data (handle , ptr as _ , size as _) } } fn get_environ (& self , handle : HANDLE) -> Result < Vec < u16 >, &'static str > { let ptr = self . Environment ; unsafe { let size = get_region_size (handle , ptr as _) ?; get_process_data (handle , ptr as _ , size as _) } } } } ; }
};
}
