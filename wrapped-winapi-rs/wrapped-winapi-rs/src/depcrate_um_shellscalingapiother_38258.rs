// Generated macro for other_38258 (other)
macro_rules! Depcrate_um_shellscalingapiother_38258 {
() => {
// Module: crate::um::shellscalingapi
// Provides: {"other_38258"}
// Dependencies: {}
extern "system" { pub fn SetProcessDpiAwareness (value : PROCESS_DPI_AWARENESS ,) -> HRESULT ; pub fn GetProcessDpiAwareness (hProcess : HANDLE , value : * mut PROCESS_DPI_AWARENESS ,) -> HRESULT ; pub fn GetDpiForMonitor (hmonitor : HMONITOR , dpiType : MONITOR_DPI_TYPE , dpiX : * mut UINT , dpiY : * mut UINT ,) -> HRESULT ; }
};
}
