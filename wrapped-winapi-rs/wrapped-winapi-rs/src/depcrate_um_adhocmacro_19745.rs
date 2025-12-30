// Generated macro for macro_19745 (macro)
macro_rules! Depcrate_um_adhocmacro_19745 {
() => {
// Module: crate::um::adhoc
// Provides: {"macro_19745"}
// Dependencies: {}
RIDL ! { # [uuid (0x8f10cc28 , 0xcf0d , 0x42a0 , 0xac , 0xbe , 0xe2 , 0xde , 0x70 , 0x07 , 0x38 , 0x4d)] interface IEnumDot11AdHocNetworks (IEnumDot11AdHocNetworksVtbl) : IUnknown (IUnknownVtbl) { fn Next (cElt : ULONG , rgElt : * mut * mut IDot11AdHocNetwork , pcEltFetched : * mut ULONG ,) -> HRESULT , fn Skip (cElt : ULONG ,) -> HRESULT , fn Reset () -> HRESULT , fn Clone (ppEnum : * mut * mut IEnumDot11AdHocNetworks ,) -> HRESULT , } }
};
}
