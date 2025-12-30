// Generated macro for macro_33758 (macro)
macro_rules! Depcrate_um_oaidlmacro_33758 {
() => {
// Module: crate::um::oaidl
// Provides: {"macro_33758"}
// Dependencies: {}
STRUCT ! { struct EXCEPINFO { wCode : WORD , wReserved : WORD , bstrSource : BSTR , bstrDescription : BSTR , bstrHelpFile : BSTR , dwHelpContext : DWORD , pvReserved : PVOID , pfnDeferredFillIn : Option < unsafe extern "system" fn (einfo : * mut EXCEPINFO ,) -> HRESULT >, scode : SCODE , } }
};
}
