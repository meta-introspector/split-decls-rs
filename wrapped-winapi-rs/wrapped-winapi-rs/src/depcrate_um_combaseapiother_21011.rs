// Generated macro for other_21011 (other)
macro_rules! Depcrate_um_combaseapiother_21011 {
() => {
// Module: crate::um::combaseapi
// Provides: {"other_21011"}
// Dependencies: {}
extern "system" { pub fn DllGetClassObject (rclsid : REFCLSID , riid : REFIID , ppv : * mut LPVOID ,) -> HRESULT ; pub fn DllCanUnloadNow () -> HRESULT ; pub fn CoTaskMemAlloc (cb : SIZE_T ,) -> LPVOID ; pub fn CoTaskMemRealloc (pv : LPVOID , cb : SIZE_T ,) -> LPVOID ; pub fn CoTaskMemFree (pv : LPVOID ,) ; pub fn CoFileTimeNow (lpFileTime : * mut FILETIME ,) -> HRESULT ; pub fn CLSIDFromProgIDEx (lpszProgID : LPCOLESTR , lpclsid : LPCLSID ,) -> HRESULT ; }
};
}
