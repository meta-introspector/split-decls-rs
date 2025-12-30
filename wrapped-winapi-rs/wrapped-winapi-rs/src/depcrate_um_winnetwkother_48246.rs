// Generated macro for other_48246 (other)
macro_rules! Depcrate_um_winnetwkother_48246 {
() => {
// Module: crate::um::winnetwk
// Provides: {"other_48246"}
// Dependencies: {}
extern "system" { pub fn WNetGetNetworkInformationA (lpProvider : LPCSTR , lpNetInfoStruct : LPNETINFOSTRUCT ,) -> DWORD ; pub fn WNetGetNetworkInformationW (lpProvider : LPCWSTR , lpNetInfoStruct : LPNETINFOSTRUCT ,) -> DWORD ; pub fn WNetGetLastErrorA (lpError : LPDWORD , lpErrorBuf : LPSTR , nErrorBufSize : DWORD , lpNameBuf : LPSTR , nNameBufSize : DWORD ,) -> DWORD ; pub fn WNetGetLastErrorW (lpError : LPDWORD , lpErrorBuf : LPWSTR , nErrorBufSize : DWORD , lpNameBuf : LPWSTR , nNameBufSize : DWORD ,) -> DWORD ; }
};
}
