// Generated macro for other_48224 (other)
macro_rules! Depcrate_um_winnetwkother_48224 {
() => {
// Module: crate::um::winnetwk
// Provides: {"other_48224"}
// Dependencies: {}
extern "system" { pub fn WNetDisconnectDialog1A (lpConnDlgStruct : LPDISCDLGSTRUCTA ,) -> DWORD ; pub fn WNetDisconnectDialog1W (lpConnDlgStruct : LPDISCDLGSTRUCTW ,) -> DWORD ; pub fn WNetOpenEnumA (dwScope : DWORD , dwType : DWORD , dwUsage : DWORD , lpNetResource : LPNETRESOURCEA , lphEnum : LPHANDLE ,) -> DWORD ; pub fn WNetOpenEnumW (dwScope : DWORD , dwType : DWORD , dwUsage : DWORD , lpNetResource : LPNETRESOURCEW , lphEnum : LPHANDLE ,) -> DWORD ; pub fn WNetEnumResourceA (hEnum : HANDLE , lpcCount : LPDWORD , lpBuffer : LPVOID , lpBufferSize : LPDWORD ,) -> DWORD ; pub fn WNetEnumResourceW (hEnum : HANDLE , lpcCount : LPDWORD , lpBuffer : LPVOID , lpBufferSize : LPDWORD ,) -> DWORD ; pub fn WNetCloseEnum (hEnum : HANDLE ,) -> DWORD ; pub fn WNetGetResourceParentA (lpNetResource : LPNETRESOURCEA , lpBuffer : LPVOID , lpcbBuffer : LPDWORD ,) -> DWORD ; pub fn WNetGetResourceParentW (lpNetResource : LPNETRESOURCEW , lpBuffer : LPVOID , lpcbBuffer : LPDWORD ,) -> DWORD ; pub fn WNetGetResourceInformationA (lpNetResource : LPNETRESOURCEA , lpBuffer : LPVOID , lpcbBuffer : LPDWORD , lplpSystem : * mut LPSTR ,) -> DWORD ; pub fn WNetGetResourceInformationW (lpNetResource : LPNETRESOURCEW , lpBuffer : LPVOID , lpcbBuffer : LPDWORD , lplpSystem : * mut LPWSTR ,) -> DWORD ; }
};
}
