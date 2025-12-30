// Generated macro for macro_19746 (macro)
macro_rules! Depcrate_um_adhocmacro_19746 {
() => {
// Module: crate::um::adhoc
// Provides: {"macro_19746"}
// Dependencies: {}
RIDL ! { # [uuid (0x8f10cc29 , 0xcf0d , 0x42a0 , 0xac , 0xbe , 0xe2 , 0xde , 0x70 , 0x07 , 0x38 , 0x4d)] interface IDot11AdHocNetwork (IDot11AdHocNetworkVtbl) : IUnknown (IUnknownVtbl) { fn GetStatus (eStatus : * mut DOT11_ADHOC_NETWORK_CONNECTION_STATUS ,) -> HRESULT , fn GetSSID (ppszwSSID : * mut LPWSTR ,) -> HRESULT , fn HasProfile (pf11d : * mut BOOLEAN ,) -> HRESULT , fn GetProfileName (ppszwProfileName : * mut LPWSTR ,) -> HRESULT , fn DeleteProfile () -> HRESULT , fn GetSignalQuality (puStrengthValue : * mut ULONG , puStrengthMax : * mut ULONG ,) -> HRESULT , fn GetSecuritySetting (pAdHocSecuritySetting : * mut * mut IDot11AdHocSecuritySettings ,) -> HRESULT , fn GetContextGuid (pContextGuid : * mut GUID ,) -> HRESULT , fn GetSignature (pSignature : * mut GUID ,) -> HRESULT , fn GetInterface (pAdHocInterface : * mut * mut IDot11AdHocInterface ,) -> HRESULT , fn Connect (Passphrase : LPCWSTR , GeographicalId : LONG , fSaveProfile : BOOLEAN , fMakeSavedProfileUserSpecific : BOOLEAN ,) -> HRESULT , fn Disconnect () -> HRESULT , } }
};
}
