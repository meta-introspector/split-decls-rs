// Generated macro for other_53856 (other)
macro_rules! Depcrate_um_winspoolother_53856 {
() => {
// Module: crate::um::winspool
// Provides: {"other_53856"}
// Dependencies: {}
extern "system" { pub fn WaitForPrinterChange (hPrinter : HANDLE , Flags : DWORD ,) -> DWORD ; pub fn FindFirstPrinterChangeNotification (hPrinter : HANDLE , fdwFilter : DWORD , fdwOptions : DWORD , pPrinterNotifyOptions : LPVOID ,) -> HANDLE ; pub fn FindNextPrinterChangeNotification (hChange : HANDLE , pdwChange : PDWORD , pPrinterNotifyOptions : LPVOID , ppPrinterNotifyInfo : * mut LPVOID ,) -> BOOL ; pub fn FreePrinterNotifyInfo (pPrinterNotifyInfo : PPRINTER_NOTIFY_INFO ,) -> BOOL ; pub fn FindClosePrinterChangeNotification (hChange : HANDLE ,) -> BOOL ; }
};
}
