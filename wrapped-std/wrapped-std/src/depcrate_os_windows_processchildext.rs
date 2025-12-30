// Generated macro for ChildExt (trait)
macro_rules! Depcrate_os_windows_processChildExt {
() => {
// Module: crate::os::windows::process
// Provides: {"ChildExt"}
// Dependencies: {}
# [unstable (feature = "windows_process_extensions_main_thread_handle" , issue = "96723")] pub trait ChildExt : Sealed { # [doc = " Extracts the main thread raw handle, without taking ownership"] # [unstable (feature = "windows_process_extensions_main_thread_handle" , issue = "96723")] fn main_thread_handle (& self) -> BorrowedHandle < '_ > ; }
};
}
