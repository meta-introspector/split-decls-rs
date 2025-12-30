// Generated macro for other_54045 (other)
macro_rules! Depcrate_um_winspoolother_54045 {
() => {
// Module: crate::um::winspool
// Provides: {"other_54045"}
// Dependencies: {}
extern "system" { pub fn ReportJobProcessingProgress (printerHandle : HANDLE , jobId : ULONG , jobOperation : EPrintXPSJobOperation , jobProgress : EPrintXPSJobProgress ,) -> HRESULT ; pub fn GetPrinterDriver2A (hWnd : HWND , hPrinter : HANDLE , pEnvironment : LPSTR , Level : DWORD , pDriverInfo : LPBYTE , cbBuf : DWORD , pcbNeeded : LPDWORD ,) -> BOOL ; pub fn GetPrinterDriver2W (hWnd : HWND , hPrinter : HANDLE , pEnvironment : LPWSTR , Level : DWORD , pDriverInfo : LPBYTE , cbBuf : DWORD , pcbNeeded : LPDWORD ,) -> BOOL ; }
};
}
