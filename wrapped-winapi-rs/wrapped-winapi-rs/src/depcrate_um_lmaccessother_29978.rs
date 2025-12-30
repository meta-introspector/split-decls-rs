// Generated macro for other_29978 (other)
macro_rules! Depcrate_um_lmaccessother_29978 {
() => {
// Module: crate::um::lmaccess
// Provides: {"other_29978"}
// Dependencies: {}
extern "system" { pub fn NetValidatePasswordPolicy (ServerName : LPCWSTR , Qualifier : LPVOID , ValidationType : NET_VALIDATE_PASSWORD_TYPE , InputArg : LPVOID , OutputArg : * mut LPVOID ,) -> NET_API_STATUS ; pub fn NetValidatePasswordPolicyFree (OutputArg : * mut LPVOID ,) -> NET_API_STATUS ; pub fn NetGetDCName (servername : LPCWSTR , domainname : LPCWSTR , bufptr : * mut LPBYTE ,) -> NET_API_STATUS ; pub fn NetGetAnyDCName (servername : LPCWSTR , domainname : LPCWSTR , bufptr : * mut LPBYTE ,) -> NET_API_STATUS ; pub fn I_NetLogonControl (ServerName : LPCWSTR , FunctionCode : DWORD , QueryLevel : DWORD , Buffer : * mut LPBYTE ,) -> NET_API_STATUS ; pub fn I_NetLogonControl2 (ServerName : LPCWSTR , FunctionCode : DWORD , QueryLevel : DWORD , Data : LPBYTE , Buffer : * mut LPBYTE ,) -> NET_API_STATUS ; }
};
}
