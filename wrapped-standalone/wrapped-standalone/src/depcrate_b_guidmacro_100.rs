// Generated macro for macro_100 (macro)
macro_rules! Depcrate_b_guidmacro_100 {
() => {
// Module: crate::b_guid
// Provides: {"macro_100"}
// Dependencies: {}
windows_link :: link ! ("ole32.dll" "system" fn CoCreateGuid (pguid : * mut GUID) -> HRESULT) ;
};
}
