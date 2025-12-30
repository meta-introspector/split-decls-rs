// Generated macro for other_43336 (other)
macro_rules! Depcrate_um_wincryptother_43336 {
() => {
// Module: crate::um::wincrypt
// Provides: {"other_43336"}
// Dependencies: {}
extern "system" { pub fn CertAddEncodedCTLToStore (hCertStore : HCERTSTORE , dwMsgAndCertEncodingType : DWORD , pbCtlEncoded : * const BYTE , cbCtlEncoded : DWORD , dwAddDisposition : DWORD , ppCtlContext : * mut PCCTL_CONTEXT ,) -> BOOL ; pub fn CertAddCTLContextToStore (hCertStore : HCERTSTORE , pCtlContext : PCCTL_CONTEXT , dwAddDisposition : DWORD , ppStoreContext : * mut PCCTL_CONTEXT ,) -> BOOL ; pub fn CertSerializeCTLStoreElement (pCtlContext : PCCTL_CONTEXT , dwFlags : DWORD , pbElement : * mut BYTE , pcbElement : * mut DWORD ,) -> BOOL ; pub fn CertDeleteCTLFromStore (pCtlContext : PCCTL_CONTEXT ,) -> BOOL ; pub fn CertAddCertificateLinkToStore (hCertStore : HCERTSTORE , pCertContext : PCCERT_CONTEXT , dwAddDisposition : DWORD , ppStoreContext : * mut PCCERT_CONTEXT ,) -> BOOL ; pub fn CertAddCRLLinkToStore (hCertStore : HCERTSTORE , pCrlContext : PCCRL_CONTEXT , dwAddDisposition : DWORD , ppStoreContext : * mut PCCRL_CONTEXT ,) -> BOOL ; pub fn CertAddCTLLinkToStore (hCertStore : HCERTSTORE , pCtlContext : PCCTL_CONTEXT , dwAddDisposition : DWORD , ppStoreContext : * mut PCCTL_CONTEXT ,) -> BOOL ; pub fn CertAddStoreToCollection (hCollectionStore : HCERTSTORE , hSiblingStore : HCERTSTORE , dwUpdateFlags : DWORD , dwPriority : DWORD ,) -> BOOL ; pub fn CertRemoveStoreFromCollection (hCollectionStore : HCERTSTORE , hSiblingStore : HCERTSTORE ,) ; pub fn CertControlStore (hCertStore : HCERTSTORE , dwFlags : DWORD , dwCtrlType : DWORD , pvCtrlPara : * const c_void ,) -> BOOL ; }
};
}
