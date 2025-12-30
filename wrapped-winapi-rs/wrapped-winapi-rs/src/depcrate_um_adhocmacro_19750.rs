// Generated macro for macro_19750 (macro)
macro_rules! Depcrate_um_adhocmacro_19750 {
() => {
// Module: crate::um::adhoc
// Provides: {"macro_19750"}
// Dependencies: {}
RIDL ! { # [uuid (0x8f10cc2d , 0xcf0d , 0x42a0 , 0xac , 0xbe , 0xe2 , 0xde , 0x70 , 0x07 , 0x38 , 0x4d)] interface IEnumDot11AdHocSecuritySettings (IEnumDot11AdHocSecuritySettingsVtbl) : IUnknown (IUnknownVtbl) { fn Next (cElt : ULONG , rgElt : * mut * mut IDot11AdHocSecuritySettings , pcEltFetched : * mut ULONG ,) -> HRESULT , fn Skip (cElt : ULONG ,) -> HRESULT , fn Reset () -> HRESULT , fn Clone (ppEnum : * mut * mut IEnumDot11AdHocSecuritySettings ,) -> HRESULT , } }
};
}
