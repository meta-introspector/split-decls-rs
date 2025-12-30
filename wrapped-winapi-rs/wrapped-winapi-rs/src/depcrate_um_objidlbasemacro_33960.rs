// Generated macro for macro_33960 (macro)
macro_rules! Depcrate_um_objidlbasemacro_33960 {
() => {
// Module: crate::um::objidlbase
// Provides: {"macro_33960"}
// Dependencies: {}
RIDL ! { # [uuid (0x000001ce , 0x0000 , 0x0000 , 0xc0 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x46)] interface IComThreadingInfo (IComThreadingInfoVtbl) : IUnknown (IUnknownVtbl) { fn GetCurrentApartmentType (pAptType : * mut APTTYPE ,) -> HRESULT , fn GetCurrentThreadType (pThreadType : * mut THDTYPE ,) -> HRESULT , fn GetCurrentLogicalThreadId (pguidLogicalThreadId : * mut GUID ,) -> HRESULT , fn SetCurrentLogicalThreadId (rguid : REFGUID ,) -> HRESULT , } }
};
}
