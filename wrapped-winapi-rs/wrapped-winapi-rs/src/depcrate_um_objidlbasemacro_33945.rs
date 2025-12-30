// Generated macro for macro_33945 (macro)
macro_rules! Depcrate_um_objidlbasemacro_33945 {
() => {
// Module: crate::um::objidlbase
// Provides: {"macro_33945"}
// Dependencies: {}
RIDL ! { # [uuid (0xdb2f3acb , 0x2f86 , 0x11d1 , 0x8e , 0x04 , 0x00 , 0xc0 , 0x4f , 0xb9 , 0x98 , 0x9a)] interface AsyncIPipeByte (AsyncIPipeByteVtbl) : IUnknown (IUnknownVtbl) { fn Begin_Pull (cRequest : ULONG ,) -> HRESULT , fn Finish_Pull (buf : * mut BYTE , pcReturned : * mut ULONG ,) -> HRESULT , fn Begin_Push (buf : * mut BYTE , cSent : ULONG ,) -> HRESULT , fn Finish_Push () -> HRESULT , } }
};
}
