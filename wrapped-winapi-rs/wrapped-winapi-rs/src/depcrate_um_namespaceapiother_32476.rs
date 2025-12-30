// Generated macro for other_32476 (other)
macro_rules! Depcrate_um_namespaceapiother_32476 {
() => {
// Module: crate::um::namespaceapi
// Provides: {"other_32476"}
// Dependencies: {}
extern "system" { pub fn CreatePrivateNamespaceW (lpPrivateNamespaceAttributes : LPSECURITY_ATTRIBUTES , lpBoundaryDescriptor : LPVOID , lpAliasPrefix : LPCWSTR ,) -> HANDLE ; pub fn OpenPrivateNamespaceW (lpBoundaryDescriptor : LPVOID , lpAliasPrefix : LPCWSTR ,) -> HANDLE ; pub fn ClosePrivateNamespace (Handle : HANDLE , Flags : ULONG ,) -> BOOLEAN ; pub fn CreateBoundaryDescriptorW (Name : LPCWSTR , Flags : ULONG ,) -> HANDLE ; pub fn AddSIDToBoundaryDescriptor (BoundaryDescriptor : * mut HANDLE , RequiredSid : PSID ,) -> BOOL ; pub fn DeleteBoundaryDescriptor (BoundaryDescriptor : HANDLE ,) -> () ; }
};
}
