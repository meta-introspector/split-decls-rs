// Generated macro for macro_33895 (macro)
macro_rules! Depcrate_um_objidlbasemacro_33895 {
() => {
// Module: crate::um::objidlbase
// Provides: {"macro_33895"}
// Dependencies: {}
RIDL ! { # [uuid (0xa5029fb6 , 0x3c34 , 0x11d1 , 0x9c , 0x99 , 0x00 , 0xc0 , 0x4f , 0xb9 , 0x98 , 0xaa)] interface IAsyncRpcChannelBuffer (IAsyncRpcChannelBufferVtbl) : IRpcChannelBuffer2 (IRpcChannelBuffer2Vtbl) { fn Send (pMsg : * mut RPCOLEMESSAGE , pSync : * mut ISynchronize , pulStatus : * mut ULONG ,) -> HRESULT , fn Receive (pMsg : * mut RPCOLEMESSAGE , pulStatus : * mut ULONG ,) -> HRESULT , fn GetDestCtxEx (pMsg : * mut RPCOLEMESSAGE , pdwDestContext : * mut DWORD , ppvDestContext : * mut * mut c_void ,) -> HRESULT , } }
};
}
