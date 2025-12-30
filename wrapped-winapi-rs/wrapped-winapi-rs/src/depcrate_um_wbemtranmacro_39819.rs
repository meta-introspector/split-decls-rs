// Generated macro for macro_39819 (macro)
macro_rules! Depcrate_um_wbemtranmacro_39819 {
() => {
// Module: crate::um::wbemtran
// Provides: {"macro_39819"}
// Dependencies: {}
RIDL ! { # [uuid (0x9ef76194 , 0x70d5 , 0x11d1 , 0xad , 0x90 , 0x00 , 0xc0 , 0x4f , 0xd8 , 0xfd , 0xff)] interface IWbemConstructClassObject (IWbemConstructClassObjectVtbl) : IUnknown (IUnknownVtbl) { fn SetInheritanceChain (lNumAntecedents : c_long , awszAntecedents : * mut LPWSTR ,) -> HRESULT , fn SetPropertyOrigin (wszPropertyName : LPCWSTR , lOriginIndex : c_long ,) -> HRESULT , fn SetMethodOrigin (wszMethodName : LPCWSTR , lOriginIndex : c_long ,) -> HRESULT , fn SetServerNamespace (wszServer : LPCWSTR , wszNamespace : LPCWSTR ,) -> HRESULT , } }
};
}
