// Generated macro for other_20988 (other)
macro_rules! Depcrate_um_combaseapiother_20988 {
() => {
// Module: crate::um::combaseapi
// Provides: {"other_20988"}
// Dependencies: {}
extern "system" { pub fn CoGetMalloc (dwMemContext : DWORD , ppMalloc : * mut LPMALLOC ,) -> HRESULT ; pub fn CreateStreamOnHGlobal (hGlobal : HGLOBAL , fDeleteOnRelease : BOOL , ppstm : * mut LPSTREAM ,) -> HRESULT ; pub fn GetHGlobalFromStream (pstm : LPSTREAM , phglobal : * mut HGLOBAL ,) -> HRESULT ; pub fn CoUninitialize () -> () ; pub fn CoGetCurrentProcess () -> DWORD ; pub fn CoInitializeEx (pvReserved : LPVOID , dwCoInit : DWORD ,) -> HRESULT ; pub fn CoGetCallerTID (lpdwTID : LPDWORD ,) -> HRESULT ; pub fn CoGetCurrentLogicalThreadId (pguid : * mut GUID ,) -> HRESULT ; pub fn CoGetContextToken (pToken : * mut ULONG_PTR ,) -> HRESULT ; pub fn CoGetDefaultContext (aptType : APTTYPE , riid : REFIID , ppv : * mut * mut c_void ,) -> HRESULT ; pub fn CoGetApartmentType (pAptType : * mut APTTYPE , pAptQualifier : * mut APTTYPEQUALIFIER ,) -> HRESULT ; }
};
}
