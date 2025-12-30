// Generated macro for other_40177 (other)
macro_rules! Depcrate_um_winbaseother_40177 {
() => {
// Module: crate::um::winbase
// Provides: {"other_40177"}
// Dependencies: {}
extern "system" { pub fn LogonUserA (lpUsername : LPCSTR , lpDomain : LPCSTR , lpPassword : LPCSTR , dwLogonType : DWORD , dwLogonProvider : DWORD , phToken : PHANDLE ,) -> BOOL ; pub fn LogonUserW (lpUsername : LPCWSTR , lpDomain : LPCWSTR , lpPassword : LPCWSTR , dwLogonType : DWORD , dwLogonProvider : DWORD , phToken : PHANDLE ,) -> BOOL ; pub fn LogonUserExA (lpUsername : LPCSTR , lpDomain : LPCSTR , lpPassword : LPCSTR , dwLogonType : DWORD , dwLogonProvider : DWORD , phToken : PHANDLE , ppLogonSid : * mut PSID , ppProfileBuffer : * mut PVOID , pdwProfileLength : LPDWORD , pQuotaLimits : PQUOTA_LIMITS ,) -> BOOL ; pub fn LogonUserExW (lpUsername : LPCWSTR , lpDomain : LPCWSTR , lpPassword : LPCWSTR , dwLogonType : DWORD , dwLogonProvider : DWORD , phToken : PHANDLE , ppLogonSid : * mut PSID , ppProfileBuffer : * mut PVOID , pdwProfileLength : LPDWORD , pQuotaLimits : PQUOTA_LIMITS ,) -> BOOL ; }
};
}
