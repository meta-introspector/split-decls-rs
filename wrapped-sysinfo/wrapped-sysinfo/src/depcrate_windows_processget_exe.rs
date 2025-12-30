// Generated macro for get_exe (function)
macro_rules! Depcrate_windows_processget_exe {
() => {
// Module: crate::windows::process
// Provides: {"get_exe"}
// Dependencies: {}
unsafe fn get_exe (process_handler : & HandleWrapper) -> Option < PathBuf > { let mut exe_buf = [0u16 ; MAX_PATH as usize + 1] ; unsafe { GetModuleFileNameExW (Some (* * process_handler) , Some (HMODULE :: default ()) , exe_buf . as_mut_slice () ,) ; Some (PathBuf :: from (null_terminated_wchar_to_string (& exe_buf))) } }
};
}
