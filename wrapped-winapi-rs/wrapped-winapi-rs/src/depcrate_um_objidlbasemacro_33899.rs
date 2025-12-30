// Generated macro for macro_33899 (macro)
macro_rules! Depcrate_um_objidlbasemacro_33899 {
() => {
// Module: crate::um::objidlbase
// Provides: {"macro_33899"}
// Dependencies: {}
RIDL ! { # [uuid (0xd5f56afc , 0x593b , 0x101a , 0xb5 , 0x69 , 0x08 , 0x00 , 0x2b , 0x2d , 0xbf , 0x7a)] interface IRpcStubBuffer (IRpcStubBufferVtbl) : IUnknown (IUnknownVtbl) { fn Connect (pUnkServer : * mut IUnknown ,) -> HRESULT , fn Disconnect () -> () , fn Invoke (_prpcmsg : * mut RPCOLEMESSAGE , _pRpcChannelBuffer : * mut IRpcChannelBuffer ,) -> HRESULT , fn IsIIDSupported (riid : REFIID ,) -> * mut IRpcStubBuffer , fn CountRefs () -> ULONG , fn DebugServerQueryInterface (ppv : * mut * mut c_void ,) -> HRESULT , fn DebugServerRelease (pv : * mut c_void ,) -> () , } }
};
}
