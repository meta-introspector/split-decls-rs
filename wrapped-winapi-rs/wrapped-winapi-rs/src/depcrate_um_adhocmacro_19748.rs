// Generated macro for macro_19748 (macro)
macro_rules! Depcrate_um_adhocmacro_19748 {
() => {
// Module: crate::um::adhoc
// Provides: {"macro_19748"}
// Dependencies: {}
RIDL ! { # [uuid (0x8f10cc2b , 0xcf0d , 0x42a0 , 0xac , 0xbe , 0xe2 , 0xde , 0x70 , 0x07 , 0x38 , 0x4d)] interface IDot11AdHocInterface (IDot11AdHocInterfaceVtbl) : IUnknown (IUnknownVtbl) { fn GetDeviceSignature (pSignature : * mut GUID ,) -> HRESULT , fn GetFriendlyName (ppszName : * mut LPWSTR ,) -> HRESULT , fn IsDot11d (pf11d : * mut BOOLEAN ,) -> HRESULT , fn IsAdHocCapable (pfAdHocCapable : * mut BOOLEAN ,) -> HRESULT , fn IsRadioOn (pfIsRadioOn : * mut BOOLEAN ,) -> HRESULT , fn GetActiveNetwork (ppNetwork : * mut * mut IDot11AdHocNetwork ,) -> HRESULT , fn GetIEnumSecuritySettings (ppEnum : * mut * mut IEnumDot11AdHocSecuritySettings ,) -> HRESULT , fn GetIEnumDot11AdHocNetworks (pFilterGuid : * mut GUID , ppEnum : * mut * mut IEnumDot11AdHocNetworks ,) -> HRESULT , fn GetStatus (pState : * mut DOT11_ADHOC_NETWORK_CONNECTION_STATUS ,) -> HRESULT , } }
};
}
