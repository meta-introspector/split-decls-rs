// Generated macro for macro_3 (macro)
macro_rules! Depcrate_bindingsmacro_3 {
() => {
// Module: crate::bindings
// Provides: {"macro_3"}
// Dependencies: {}
windows_link :: link ! ("advapi32.dll" "system" fn SetServiceStatus (hservicestatus : SERVICE_STATUS_HANDLE , lpservicestatus : * const SERVICE_STATUS) -> BOOL) ;
};
}
