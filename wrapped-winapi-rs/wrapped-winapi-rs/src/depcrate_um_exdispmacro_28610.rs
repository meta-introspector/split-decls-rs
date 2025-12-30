// Generated macro for macro_28610 (macro)
macro_rules! Depcrate_um_exdispmacro_28610 {
() => {
// Module: crate::um::exdisp
// Provides: {"macro_28610"}
// Dependencies: {}
RIDL ! { # [uuid (0x0002df05 , 0x0000 , 0x0000 , 0xc0 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x46)] interface IWebBrowserApp (IWebBrowserAppVtbl) : IWebBrowser (IWebBrowserVtbl) { fn Quit () -> HRESULT , fn ClientToWindow (pcx : * mut INT , pcy : * mut INT ,) -> HRESULT , fn PutProperty (Property : BSTR , vtValue : VARIANT ,) -> HRESULT , fn GetProperty (Property : BSTR , pvtValue : * mut VARIANT ,) -> HRESULT , fn get_Name (Name : * mut BSTR ,) -> HRESULT , fn get_HWND (pHWND : * mut SHANDLE_PTR ,) -> HRESULT , fn get_FullName (FullName : * mut BSTR ,) -> HRESULT , fn get_Path (Path : * mut BSTR ,) -> HRESULT , fn get_Visible (pBool : * mut VARIANT_BOOL ,) -> HRESULT , fn put_Visible (Value : VARIANT_BOOL ,) -> HRESULT , fn get_StatusBar (pBool : * mut VARIANT_BOOL ,) -> HRESULT , fn put_StatusBar (Value : VARIANT_BOOL ,) -> HRESULT , fn get_StatusText (StatusText : * mut BSTR ,) -> HRESULT , fn put_StatusText (StatusText : BSTR ,) -> HRESULT , fn get_ToolBar (Value : * mut INT ,) -> HRESULT , fn put_ToolBar (Value : INT ,) -> HRESULT , fn get_MenuBar (Value : * mut VARIANT_BOOL ,) -> HRESULT , fn put_MenuBar (Value : VARIANT_BOOL ,) -> HRESULT , fn get_FullScreen (pbFullScreen : * mut VARIANT_BOOL ,) -> HRESULT , fn put_FullScreen (bFullScreen : VARIANT_BOOL ,) -> HRESULT , } }
};
}
