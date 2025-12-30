// Generated macro for macro_33893 (macro)
macro_rules! Depcrate_um_objidlbasemacro_33893 {
() => {
// Module: crate::um::objidlbase
// Provides: {"macro_33893"}
// Dependencies: {}
RIDL ! { # [uuid (0xd5f56b60 , 0x593b , 0x101a , 0xb5 , 0x69 , 0x08 , 0x00 , 0x2b , 0x2d , 0xbf , 0x7a)] interface IRpcChannelBuffer (IRpcChannelBufferVtbl) : IUnknown (IUnknownVtbl) { fn GetBuffer (pMessage : * mut RPCOLEMESSAGE , riid : REFIID ,) -> HRESULT , fn SendReceive (pMessage : * mut RPCOLEMESSAGE , pStatus : * mut ULONG ,) -> HRESULT , fn FreeBuffer (pMessage : * mut RPCOLEMESSAGE ,) -> HRESULT , fn GetDestCtx (pdwDestContext : * mut DWORD , ppvDestContext : * mut * mut c_void ,) -> HRESULT , fn IsConnected () -> HRESULT , } }
};
}
