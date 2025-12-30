// Generated macro for macro_33800 (macro)
macro_rules! Depcrate_um_oaidlmacro_33800 {
() => {
// Module: crate::um::oaidl
// Provides: {"macro_33800"}
// Dependencies: {}
RIDL ! { # [uuid (0x00020402 , 0x0000 , 0x0000 , 0xc0 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x46)] interface ITypeLib (ITypeLibVtbl) : IUnknown (IUnknownVtbl) { fn GetTypeInfoCount () -> UINT , fn GetTypeInfo (index : UINT , ppTInfo : * mut * mut ITypeInfo ,) -> HRESULT , fn GetTypeInfoType (index : UINT , pTKind : * mut TYPEKIND ,) -> HRESULT , fn GetTypeInfoOfGuid (guid : REFGUID , ppTInfo : * mut * mut ITypeInfo ,) -> HRESULT , fn GetLibAttr (ppTLibAttr : * mut * mut TLIBATTR ,) -> HRESULT , fn GetTypeComp (ppTComp : * mut * mut ITypeComp ,) -> HRESULT , fn GetDocumentation (index : INT , pbstrName : * mut BSTR , pBstrDocString : * mut BSTR , pdwHelpContext : * mut DWORD , pBstrHelpFile : * mut BSTR ,) -> HRESULT , fn IsName (szNameBuf : LPOLESTR , lHashVal : ULONG , pfName : * mut BOOL ,) -> HRESULT , fn FindName (szNameBuf : LPOLESTR , lHashVal : ULONG , ppTInfo : * mut * mut ITypeInfo , rgMemId : * mut MEMBERID , pcFound : * mut USHORT ,) -> HRESULT , fn ReleaseTLibAttr (pTLibAttr : * const TLIBATTR ,) -> HRESULT , } }
};
}
