// Generated macro for macro_33896 (macro)
macro_rules! Depcrate_um_objidlbasemacro_33896 {
() => {
// Module: crate::um::objidlbase
// Provides: {"macro_33896"}
// Dependencies: {}
RIDL ! { # [uuid (0x25b15600 , 0x0115 , 0x11d0 , 0xbf , 0x0d , 0x00 , 0xaa , 0x00 , 0xb8 , 0xdf , 0xd2)] interface IRpcChannelBuffer3 (IRpcChannelBuffer3Vtbl) : IRpcChannelBuffer2 (IRpcChannelBuffer2Vtbl) { fn Send (pMsg : * mut RPCOLEMESSAGE , pulStatus : * mut ULONG ,) -> HRESULT , fn Receive (pMsg : * mut RPCOLEMESSAGE , ulSize : ULONG , pulStatus : * mut ULONG ,) -> HRESULT , fn Cancel (pMsg : * mut RPCOLEMESSAGE ,) -> HRESULT , fn GetCallContext (pMsg : * mut RPCOLEMESSAGE , riid : REFIID , pInterface : * mut * mut c_void ,) -> HRESULT , fn GetDestCtxEx (pMsg : * mut RPCOLEMESSAGE , pdwDestContext : * mut DWORD , ppvDestContext : * mut * mut c_void ,) -> HRESULT , fn GetState (pMsg : * mut RPCOLEMESSAGE , pState : * mut DWORD ,) -> HRESULT , fn RegisterAsync (pMsg : * mut RPCOLEMESSAGE , pAsyncMgr : * mut IAsyncManager ,) -> HRESULT , } }
};
}
