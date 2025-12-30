// Generated macro for impl_2010 (impl)
macro_rules! Depcrate_os_windows_processimpl_2010 {
() => {
// Module: crate::os::windows::process
// Provides: {"impl_2010"}
// Dependencies: {}
# [unstable (feature = "windows_process_extensions_main_thread_handle" , issue = "96723")] impl ChildExt for process :: Child { fn main_thread_handle (& self) -> BorrowedHandle < '_ > { self . handle . main_thread_handle () } }
};
}
