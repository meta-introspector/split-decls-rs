// Generated macro for macro_33781 (macro)
macro_rules! Depcrate_um_oaidlmacro_33781 {
() => {
// Module: crate::um::oaidl
// Provides: {"macro_33781"}
// Dependencies: {}
RIDL ! { # [uuid (0x00020405 , 0x0000 , 0x0000 , 0xc0 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x46)] interface ICreateTypeInfo (ICreateTypeInfoVtbl) : IUnknown (IUnknownVtbl) { fn SetGuid (guid : REFGUID ,) -> HRESULT , fn SetTypeFlags (uTypeFlags : UINT ,) -> HRESULT , fn SetDocString (pStrDoc : LPOLESTR ,) -> HRESULT , fn SetHelpContext (dwHelpContext : DWORD ,) -> HRESULT , fn SetVersion (wMajorVerNum : WORD , wMinorVerNum : WORD ,) -> HRESULT , fn AddRefTypeInfo (pTInfo : * mut ITypeInfo ,) -> HRESULT , fn AddFuncDesc (index : UINT , pFuncDesc : * mut FUNCDESC ,) -> HRESULT , fn SetImplTypeFlags (index : UINT , implTypeFlags : INT ,) -> HRESULT , fn SetAlignment (cbAlignment : WORD ,) -> HRESULT , fn SetSchema (pStrSchema : LPOLESTR ,) -> HRESULT , fn AddVarDesc (index : UINT , pVarDesc : * mut VARDESC ,) -> HRESULT , fn SetFuncAndParamNames (index : UINT , rgszNames : * mut LPOLESTR , cNames : UINT ,) -> HRESULT , fn SetVarName (index : UINT , szName : LPOLESTR ,) -> HRESULT , fn SetTypeDescAlias (pTDescAlias : * mut TYPEDESC ,) -> HRESULT , fn DefineFuncAsDllEntry (index : UINT , szDllName : LPOLESTR , szProcName : LPOLESTR ,) -> HRESULT , fn SetFuncDocString (index : UINT , szDocString : LPOLESTR ,) -> HRESULT , fn SetVarDocString (index : UINT , szDocString : LPOLESTR ,) -> HRESULT , fn SetFuncHelpContext (index : UINT , dwHelpContext : DWORD ,) -> HRESULT , fn SetVarHelpContext (index : UINT , dwHelpContext : DWORD ,) -> HRESULT , fn SetMops (index : UINT , bstrMops : BSTR ,) -> HRESULT , fn SetTypeIdldesc (pIdlDesc : * mut IDLDESC ,) -> HRESULT , fn LayOut () -> HRESULT , } }
};
}
