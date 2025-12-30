// Generated macro for macro_27441 (macro)
macro_rules! Depcrate_um_dispexmacro_27441 {
() => {
// Module: crate::um::dispex
// Provides: {"macro_27441"}
// Dependencies: {}
RIDL ! { # [uuid (0xa6ef9860 , 0xc720 , 0x11d0 , 0x93 , 0x37 , 0x00 , 0xa0 , 0xc9 , 0x0d , 0xca , 0xa9)] interface IDispatchEx (IDispatchExVtbl) : IDispatch (IDispatchVtbl) { fn GetDispID (bstrName : BSTR , grfdex : DWORD , pid : * mut DISPID ,) -> HRESULT , fn InvokeEx (id : DISPID , lcid : LCID , wFlags : WORD , pdp : * mut DISPPARAMS , pvarRes : * mut VARIANT , pei : * mut EXCEPINFO , pspCaller : * mut IServiceProvider ,) -> HRESULT , fn DeleteMemberByName (bstrName : BSTR , grfdex : DWORD ,) -> HRESULT , fn DeleteMemberByDispID (id : DISPID ,) -> HRESULT , fn GetMemberProperties (id : DISPID , grfdexFetch : DWORD , pgrfdex : * mut DWORD ,) -> HRESULT , fn GetMemberName (id : DISPID , pbstrName : * mut BSTR ,) -> HRESULT , fn GetNextDispID (grfdex : DWORD , id : DISPID , pid : * mut DISPID ,) -> HRESULT , fn GetNameSpaceParent (ppunk : * mut * mut IUnknown ,) -> HRESULT , } }
};
}
