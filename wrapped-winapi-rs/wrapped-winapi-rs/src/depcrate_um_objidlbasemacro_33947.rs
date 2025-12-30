// Generated macro for macro_33947 (macro)
macro_rules! Depcrate_um_objidlbasemacro_33947 {
() => {
// Module: crate::um::objidlbase
// Provides: {"macro_33947"}
// Dependencies: {}
RIDL ! { # [uuid (0xdb2f3acd , 0x2f86 , 0x11d1 , 0x8e , 0x04 , 0x00 , 0xc0 , 0x4f , 0xb9 , 0x98 , 0x9a)] interface AsyncIPipeLong (AsyncIPipeLongVtbl) : IUnknown (IUnknownVtbl) { fn Begin_Pull (cRequest : ULONG ,) -> HRESULT , fn Finish_Pull (buf : * mut LONG , pcReturned : * mut ULONG ,) -> HRESULT , fn Begin_Push (buf : * mut LONG , cSent : ULONG ,) -> HRESULT , fn Finish_Push () -> HRESULT , } }
};
}
