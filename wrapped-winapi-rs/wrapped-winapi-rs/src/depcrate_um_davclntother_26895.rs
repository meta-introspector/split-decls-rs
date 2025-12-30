// Generated macro for other_26895 (other)
macro_rules! Depcrate_um_davclntother_26895 {
() => {
// Module: crate::um::davclnt
// Provides: {"other_26895"}
// Dependencies: {}
extern "system" { pub fn DavAddConnection (ConnectionHandle : * mut HANDLE , RemoteName : LPCWSTR , UserName : LPCWSTR , Password : LPCWSTR , ClientCert : PBYTE , CertSize : DWORD ,) -> DWORD ; pub fn DavDeleteConnection (ConnectionHandle : HANDLE ,) -> DWORD ; pub fn DavGetUNCFromHTTPPath (HttpPath : LPCWSTR , UncPath : LPWSTR , lpSize : LPDWORD ,) -> DWORD ; pub fn DavGetHTTPFromUNCPath (UncPath : LPCWSTR , HttpPath : LPWSTR , lpSize : LPDWORD ,) -> DWORD ; pub fn DavGetTheLockOwnerOfTheFile (FileName : LPCWSTR , LockOwnerName : PWSTR , LockOwnerNameLengthInBytes : PULONG ,) -> DWORD ; pub fn DavGetExtendedError (hFile : HANDLE , ExtError : * mut DWORD , ExtErrorString : LPWSTR , cChSize : * mut DWORD ,) -> DWORD ; pub fn DavFlushFile (hFile : HANDLE ,) -> DWORD ; pub fn DavInvalidateCache (URLName : LPWSTR ,) -> DWORD ; pub fn DavCancelConnectionsToServer (URLName : LPWSTR , fForce : BOOL ,) -> DWORD ; pub fn DavRegisterAuthCallback (CallBack : PFNDAVAUTHCALLBACK , Version : ULONG ,) -> OPAQUE_HANDLE ; pub fn DavUnregisterAuthCallback (hCallback : OPAQUE_HANDLE ,) ; }
};
}
