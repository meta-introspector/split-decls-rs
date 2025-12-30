// Generated macro for other_31774 (other)
macro_rules! Depcrate_um_lsalookupother_31774 {
() => {
// Module: crate::um::lsalookup
// Provides: {"other_31774"}
// Dependencies: {}
extern "C" { pub fn LsaLookupOpenLocalPolicy (ObjectAttributes : PLSA_OBJECT_ATTRIBUTES , AccessMask : ACCESS_MASK , PolicyHandle : PLSA_LOOKUP_HANDLE ,) -> NTSTATUS ; pub fn LsaLookupClose (ObjectHandle : LSA_LOOKUP_HANDLE ,) -> NTSTATUS ; pub fn LsaLookupTranslateSids (PolicyHandle : LSA_LOOKUP_HANDLE , Count : ULONG , Sids : * mut PSID , ReferencedDomains : * mut PLSA_REFERENCED_DOMAIN_LIST , Names : * mut PLSA_TRANSLATED_NAME ,) -> NTSTATUS ; pub fn LsaLookupTranslateNames (PolicyHandle : LSA_LOOKUP_HANDLE , Flags : ULONG , Count : ULONG , Names : PLSA_UNICODE_STRING , ReferencedDomains : * mut PLSA_REFERENCED_DOMAIN_LIST , Sids : * mut PLSA_TRANSLATED_SID2 ,) -> NTSTATUS ; pub fn LsaLookupGetDomainInfo (PolicyHandle : LSA_LOOKUP_HANDLE , DomainInfoClass : LSA_LOOKUP_DOMAIN_INFO_CLASS , DomainInfo : * mut PVOID ,) -> NTSTATUS ; pub fn LsaLookupFreeMemory (Buffer : PVOID ,) -> NTSTATUS ; }
};
}
