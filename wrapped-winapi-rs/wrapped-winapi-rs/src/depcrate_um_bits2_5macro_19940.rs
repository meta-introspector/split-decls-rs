// Generated macro for macro_19940 (macro)
macro_rules! Depcrate_um_bits2_5macro_19940 {
() => {
// Module: crate::um::bits2_5
// Provides: {"macro_19940"}
// Dependencies: {}
RIDL ! { # [uuid (0xf1bd1079 , 0x9f01 , 0x4bdc , 0x80 , 0x36 , 0xf0 , 0x9b , 0x70 , 0x09 , 0x50 , 0x66)] interface IBackgroundCopyJobHttpOptions (IBackgroundCopyJobHttpOptionsVtbl) : IUnknown (IUnknownVtbl) { fn SetClientCertificateByID (StoreLocation : BG_CERT_STORE_LOCATION , StoreName : LPCWSTR , pCertHashBlob : * mut byte ,) -> HRESULT , fn SetClientCertificateByName (StoreLocation : BG_CERT_STORE_LOCATION , StoreName : LPCWSTR , SubjectName : LPCWSTR ,) -> HRESULT , fn RemoveClientCertificate () -> HRESULT , fn GetClientCertificate (pStoreLocation : * mut BG_CERT_STORE_LOCATION , pStoreName : * mut LPWSTR , ppCertHashBlob : * mut * mut byte , pSubjectName : * mut LPWSTR ,) -> HRESULT , fn SetCustomHeaders (RequestHeaders : LPCWSTR ,) -> HRESULT , fn GetCustomHeaders (pRequestHeaders : * mut LPWSTR ,) -> HRESULT , fn SetSecurityFlags (Flags : ULONG ,) -> HRESULT , fn GetSecurityFlags (pFlags : * mut ULONG ,) -> HRESULT , } }
};
}
