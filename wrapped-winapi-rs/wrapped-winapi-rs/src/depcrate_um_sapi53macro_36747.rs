// Generated macro for macro_36747 (macro)
macro_rules! Depcrate_um_sapi53macro_36747 {
() => {
// Module: crate::um::sapi53
// Provides: {"macro_36747"}
// Dependencies: {}
RIDL ! { # [uuid (0xb9ac5783 , 0xfcd0 , 0x4b21 , 0xb1 , 0x19 , 0xb4 , 0xf8 , 0xda , 0x8f , 0xd2 , 0xc3)] interface ISpeechResourceLoader (ISpeechResourceLoaderVtbl) : IDispatch (IDispatchVtbl) { fn LoadResource (bstrResourceUri : BSTR , fAlwaysReload : VARIANT_BOOL , pStream : * mut * mut IUnknown , pbstrMIMEType : * mut BSTR , pfModified : * mut VARIANT_BOOL , pbstrRedirectUrl : * mut BSTR ,) -> HRESULT , fn GetLocalCopy (bstrResourceUri : BSTR , pbstrLocalPath : * mut BSTR , pbstrMIMEType : * mut BSTR , pbstrRedirectUrl : * mut BSTR ,) -> HRESULT , fn ReleaseLocalCopy (pbstrLocalPath : BSTR ,) -> HRESULT , } }
};
}
