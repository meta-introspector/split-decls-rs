// Generated macro for other_43440 (other)
macro_rules! Depcrate_um_wincryptother_43440 {
() => {
// Module: crate::um::wincrypt
// Provides: {"other_43440"}
// Dependencies: {}
extern "system" { pub fn CertVerifyTimeValidity (pTimeToVerify : LPFILETIME , pCertInfo : PCERT_INFO ,) -> LONG ; pub fn CertVerifyCRLTimeValidity (pTimeToVerify : LPFILETIME , pCrlInfo : PCRL_INFO ,) -> LONG ; pub fn CertVerifyValidityNesting (pSubjectInfo : PCERT_INFO , pIssuerInfo : PCERT_INFO ,) -> BOOL ; pub fn CertVerifyCRLRevocation (dwCertEncodingType : DWORD , pCertId : PCERT_INFO , cCrlInfo : DWORD , rgpCrlInfo : * mut PCRL_INFO ,) -> BOOL ; pub fn CertAlgIdToOID (dwAlgId : DWORD ,) -> LPCSTR ; pub fn CertOIDToAlgId (pszObjId : LPCSTR ,) -> DWORD ; pub fn CertFindExtension (pszObjId : LPCSTR , cExtensions : DWORD , rgExtensions : * mut CERT_EXTENSION ,) -> PCERT_EXTENSION ; pub fn CertFindAttribute (pszObjId : LPCSTR , cAttr : DWORD , rgAttr : * mut CRYPT_ATTRIBUTE ,) -> PCRYPT_ATTRIBUTE ; pub fn CertFindRDNAttr (pszObjId : LPCSTR , pName : PCERT_NAME_INFO ,) -> PCERT_RDN_ATTR ; pub fn CertGetIntendedKeyUsage (dwCertEncodingType : DWORD , pCertInfo : PCERT_INFO , pbKeyUsage : * mut BYTE , cbKeyUsage : DWORD ,) -> BOOL ; }
};
}
