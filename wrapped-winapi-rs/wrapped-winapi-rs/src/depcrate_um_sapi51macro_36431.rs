// Generated macro for macro_36431 (macro)
macro_rules! Depcrate_um_sapi51macro_36431 {
() => {
// Module: crate::um::sapi51
// Provides: {"macro_36431"}
// Dependencies: {}
RIDL ! { # [uuid (0xbe7a9cce , 0x5f9e , 0x11d2 , 0x96 , 0x0f , 0x00 , 0xc0 , 0x4f , 0x8e , 0xe6 , 0x28)] interface ISpEventSource (ISpEventSourceVtbl) : ISpNotifySource (ISpNotifySourceVtbl) { fn SetInterest (ullEventInterest : ULONGLONG , ullQueuedInterest : ULONGLONG ,) -> HRESULT , fn GetEvents (ulCount : ULONG , pEventArray : * mut SPEVENT , pulFetched : * mut ULONG ,) -> HRESULT , fn GetInfo (pInfo : * mut SPEVENTSOURCEINFO ,) -> HRESULT , } }
};
}
