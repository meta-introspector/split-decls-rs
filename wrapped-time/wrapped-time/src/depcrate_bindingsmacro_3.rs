// Generated macro for macro_3 (macro)
macro_rules! Depcrate_bindingsmacro_3 {
() => {
// Module: crate::bindings
// Provides: {"macro_3"}
// Dependencies: {}
windows_link :: link ! ("kernel32.dll" "system" fn FileTimeToSystemTime (lpfiletime : * const FILETIME , lpsystemtime : * mut SYSTEMTIME) -> BOOL) ;
};
}
