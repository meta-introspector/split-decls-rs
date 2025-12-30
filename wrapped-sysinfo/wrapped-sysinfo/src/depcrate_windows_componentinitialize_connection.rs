// Generated macro for initialize_connection (function)
macro_rules! Depcrate_windows_componentinitialize_connection {
() => {
// Module: crate::windows::component
// Provides: {"initialize_connection"}
// Dependencies: {}
unsafe fn initialize_connection () -> Result < () , () > { if unsafe { CoInitializeEx (None , Default :: default ()) } . is_err () { sysinfo_debug ! ("Failed to initialize connection") ; Err (()) } else { Ok (()) } }
};
}
