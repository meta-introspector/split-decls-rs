// Generated macro for macro_36432 (macro)
macro_rules! Depcrate_um_sapi51macro_36432 {
() => {
// Module: crate::um::sapi51
// Provides: {"macro_36432"}
// Dependencies: {}
RIDL ! { # [uuid (0xbe7a9cc9 , 0x5f9e , 0x11d2 , 0x96 , 0x0f , 0x00 , 0xc0 , 0x4f , 0x8e , 0xe6 , 0x28)] interface ISpEventSink (ISpEventSinkVtbl) : IUnknown (IUnknownVtbl) { fn AddEvents (pEventArray : * const SPEVENT , ulCount : ULONG ,) -> HRESULT , fn GetEventInterest (pullEventInterest : * mut ULONGLONG ,) -> HRESULT , } }
};
}
