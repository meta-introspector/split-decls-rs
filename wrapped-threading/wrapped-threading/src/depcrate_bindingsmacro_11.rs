// Generated macro for macro_11 (macro)
macro_rules! Depcrate_bindingsmacro_11 {
() => {
// Module: crate::bindings
// Provides: {"macro_11"}
// Dependencies: {}
windows_link :: link ! ("kernel32.dll" "system" fn TrySubmitThreadpoolCallback (pfns : PTP_SIMPLE_CALLBACK , pv : * mut core :: ffi :: c_void , pcbe : * const TP_CALLBACK_ENVIRON_V3) -> BOOL) ;
};
}
