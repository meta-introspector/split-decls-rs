// Generated macro for macro_28609 (macro)
macro_rules! Depcrate_um_exdispmacro_28609 {
() => {
// Module: crate::um::exdisp
// Provides: {"macro_28609"}
// Dependencies: {}
RIDL ! { # [uuid (0xeab22ac1 , 0x30c1 , 0x11cf , 0xa7 , 0xeb , 0x00 , 0x00 , 0xc0 , 0x5b , 0xae , 0x0b)] interface IWebBrowser (IWebBrowserVtbl) : IDispatch (IDispatchVtbl) { fn GoBack () -> HRESULT , fn GoForward () -> HRESULT , fn GoHome () -> HRESULT , fn GoSearch () -> HRESULT , fn Navigate (URL : BSTR , Flags : * const VARIANT , TargetFrameName : * const VARIANT , PostData : * const VARIANT , Headers : * const VARIANT ,) -> HRESULT , fn Refresh () -> HRESULT , fn Refresh2 (Level : * const VARIANT ,) -> HRESULT , fn Stop () -> HRESULT , fn get_Application (ppDisp : * mut * mut IDispatch ,) -> HRESULT , fn get_Parent (ppDisp : * mut * mut IDispatch ,) -> HRESULT , fn get_Container (ppDisp : * mut * mut IDispatch ,) -> HRESULT , fn get_Document (ppDisp : * mut * mut IDispatch ,) -> HRESULT , fn get_TopLevelContainer (pBool : * mut VARIANT_BOOL ,) -> HRESULT , fn get_Type (Type : * mut BSTR ,) -> HRESULT , fn get_Left (pl : * mut LONG ,) -> HRESULT , fn put_Left (Left : LONG ,) -> HRESULT , fn get_Top (pl : * mut LONG ,) -> HRESULT , fn put_Top (Top : LONG ,) -> HRESULT , fn get_Width (pl : * mut LONG ,) -> HRESULT , fn put_Width (Width : LONG ,) -> HRESULT , fn get_Height (pl : * mut LONG ,) -> HRESULT , fn put_Height (Height : LONG ,) -> HRESULT , fn get_LocationName (LocationName : * mut BSTR ,) -> HRESULT , fn get_LocationURL (LocationURL : * mut BSTR ,) -> HRESULT , fn get_Busy (pBool : * mut VARIANT_BOOL ,) -> HRESULT , } }
};
}
