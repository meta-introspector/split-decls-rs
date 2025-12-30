// Generated macro for macro_39764 (macro)
macro_rules! Depcrate_um_wbemprovmacro_39764 {
() => {
// Module: crate::um::wbemprov
// Provides: {"macro_39764"}
// Dependencies: {}
RIDL ! { # [uuid (0x580acaf8 , 0xfa1c , 0x11d0 , 0xad , 0x72 , 0x00 , 0xc0 , 0x4f , 0xd8 , 0xfd , 0xff)] interface IWbemEventProviderQuerySink (IWbemEventProviderQuerySinkVtbl) : IUnknown (IUnknownVtbl) { fn NewQuery (dwId : c_ulong , wszQueryLanguage : WBEM_WSTR , wszQuery : WBEM_WSTR ,) -> HRESULT , fn CancelQuery (dwId : c_ulong ,) -> HRESULT , } }
};
}
