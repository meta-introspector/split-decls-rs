// Generated macro for initialize_security (function)
macro_rules! Depcrate_windows_componentinitialize_security {
() => {
// Module: crate::windows::component
// Provides: {"initialize_security"}
// Dependencies: {}
unsafe fn initialize_security () -> Result < () , () > { if unsafe { CoInitializeSecurity (Some (PSECURITY_DESCRIPTOR :: default ()) , - 1 , None , None , RPC_C_AUTHN_LEVEL_DEFAULT , RPC_C_IMP_LEVEL_IMPERSONATE , None , EOAC_NONE , None ,) } . is_err () { sysinfo_debug ! ("Failed to initialize security") ; Err (()) } else { Ok (()) } }
};
}
