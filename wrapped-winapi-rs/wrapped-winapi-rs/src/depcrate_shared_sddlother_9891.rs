// Generated macro for other_9891 (other)
macro_rules! Depcrate_shared_sddlother_9891 {
() => {
// Module: crate::shared::sddl
// Provides: {"other_9891"}
// Dependencies: {}
extern "system" { pub fn ConvertSidToStringSidA (Sid : PSID , StringSid : * mut LPSTR ,) -> BOOL ; pub fn ConvertSidToStringSidW (Sid : PSID , StringSid : * mut LPWSTR ,) -> BOOL ; pub fn ConvertStringSidToSidA (StringSid : LPCSTR , Sid : * mut PSID ,) -> BOOL ; pub fn ConvertStringSidToSidW (StringSid : LPCWSTR , Sid : * mut PSID ,) -> BOOL ; pub fn ConvertStringSecurityDescriptorToSecurityDescriptorA (StringSecurityDescriptor : LPCSTR , StringSDRevision : DWORD , SecurityDescriptor : * mut PSECURITY_DESCRIPTOR , SecurityDescriptorSize : PULONG ,) -> BOOL ; pub fn ConvertStringSecurityDescriptorToSecurityDescriptorW (StringSecurityDescriptor : LPCWSTR , StringSDRevision : DWORD , SecurityDescriptor : * mut PSECURITY_DESCRIPTOR , SecurityDescriptorSize : PULONG ,) -> BOOL ; pub fn ConvertSecurityDescriptorToStringSecurityDescriptorA (SecurityDescriptor : PSECURITY_DESCRIPTOR , RequestedStringSDRevision : DWORD , SecurityInformation : SECURITY_INFORMATION , StringSecurityDescriptor : * mut LPSTR , StringSecurityDescriptorLen : PULONG ,) -> BOOL ; pub fn ConvertSecurityDescriptorToStringSecurityDescriptorW (SecurityDescriptor : PSECURITY_DESCRIPTOR , RequestedStringSDRevision : DWORD , SecurityInformation : SECURITY_INFORMATION , StringSecurityDescriptor : * mut LPWSTR , StringSecurityDescriptorLen : PULONG ,) -> BOOL ; }
};
}
