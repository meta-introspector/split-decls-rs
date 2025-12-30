// Generated macro for macro_36405 (macro)
macro_rules! Depcrate_um_sapi51macro_36405 {
() => {
// Module: crate::um::sapi51
// Provides: {"macro_36405"}
// Dependencies: {}
RIDL ! { # [uuid (0x14056589 , 0xe16c , 0x11d2 , 0xbb , 0x90 , 0x00 , 0xc0 , 0x4f , 0x8e , 0xe6 , 0xc0)] interface ISpObjectToken (ISpObjectTokenVtbl) : ISpDataKey (ISpDataKeyVtbl) { fn SetId (pszCategoryId : LPCWSTR , pszTokenId : LPCWSTR , fCreateIfNotExist : BOOL ,) -> HRESULT , fn GetId (ppszCoMemTokenId : * mut LPWSTR ,) -> HRESULT , fn GetCategory (ppTokenCategory : * mut * mut ISpObjectTokenCategory ,) -> HRESULT , fn CreateInstance (pUnkOuter : * mut IUnknown , dwClsContext : DWORD , riid : REFIID , ppvObject : * mut * mut c_void ,) -> HRESULT , fn GetStorageFileName (clsidCaller : REFCLSID , pszValueName : LPCWSTR , pszFileNameSpecifier : LPCWSTR , nFolder : ULONG , ppszFilePath : * mut LPWSTR ,) -> HRESULT , fn RemoveStorageFileName (pszKeyName : LPCWSTR , fDeleteFile : BOOL ,) -> HRESULT , fn Remove (pclsidCaller : * const CLSID ,) -> HRESULT , fn IsUISupported (pszTypeOfUI : LPCWSTR , pvExtraData : * mut c_void , cbExtraData : ULONG , punkObject : * mut IUnknown , pfSupported : * mut BOOL ,) -> HRESULT , fn DisplayUI (hwndParent : HWND , pszTitle : LPCWSTR , pszTypeOfUI : LPCWSTR , pvExtraData : * mut c_void , cbExtraData : ULONG , punkObject : * mut IUnknown ,) -> HRESULT , fn MatchesAttributes (pszAttributes : LPCWSTR , pfMatches : * mut BOOL ,) -> HRESULT , } }
};
}
