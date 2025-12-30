// Generated macro for other_54048 (other)
macro_rules! Depcrate_um_winspoolother_54048 {
() => {
// Module: crate::um::winspool
// Provides: {"other_54048"}
// Dependencies: {}
extern "system" { pub fn GetPrintExecutionData (pData : * mut PRINT_EXECUTION_DATA ,) -> BOOL ; pub fn GetJobNamedPropertyValue (hPrinter : HANDLE , JobId : DWORD , pszName : PCWSTR , pValue : * mut PrintPropertyValue ,) -> DWORD ; pub fn FreePrintPropertyValue (pValue : * mut PrintPropertyValue ,) ; pub fn FreePrintNamedPropertyArray (cProperties : DWORD , ppProperties : * mut * mut PrintNamedProperty ,) ; pub fn SetJobNamedProperty (hPrinter : HANDLE , JobId : DWORD , pProperty : * const PrintNamedProperty ,) -> DWORD ; pub fn DeleteJobNamedProperty (hPrinter : HANDLE , JobId : DWORD , pszName : PCWSTR ,) -> DWORD ; pub fn EnumJobNamedProperties (hPrinter : HANDLE , JobId : DWORD , pcProperties : * mut DWORD , ppProperties : * mut * mut PrintNamedProperty ,) -> DWORD ; pub fn GetPrintOutputInfo (hWnd : HWND , pszPrinter : PCWSTR , phFile : * mut HANDLE , ppszOutputFile : * mut PWSTR ,) -> HRESULT ; }
};
}
