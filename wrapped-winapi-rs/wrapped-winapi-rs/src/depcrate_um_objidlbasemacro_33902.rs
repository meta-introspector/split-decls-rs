// Generated macro for macro_33902 (macro)
macro_rules! Depcrate_um_objidlbasemacro_33902 {
() => {
// Module: crate::um::objidlbase
// Provides: {"macro_33902"}
// Dependencies: {}
RIDL ! { # [uuid (0x1008c4a0 , 0x7613 , 0x11cf , 0x9a , 0xf1 , 0x00 , 0x20 , 0xaf , 0x6e , 0x72 , 0xf4)] interface IChannelHook (IChannelHookVtbl) : IUnknown (IUnknownVtbl) { fn ClientGetSize (uExtent : REFGUID , riid : REFIID , pDataSize : * mut ULONG ,) -> () , fn ClientFillBuffer (uExtent : REFGUID , riid : REFIID , pDataSize : * mut ULONG , pDataBuffer : * mut c_void ,) -> () , fn ClientNotify (uExtent : REFGUID , riid : REFIID , cbDataSize : ULONG , pDataBuffer : * mut c_void , lDataRep : DWORD , hrFault : HRESULT ,) -> () , fn ServerNotify (uExtent : REFGUID , riid : REFIID , cbDataSize : ULONG , pDataBuffer : * mut c_void , lDataRep : DWORD ,) -> () , fn ServerGetSize (uExtent : REFGUID , riid : REFIID , hrFault : HRESULT , pDataSize : * mut ULONG ,) -> () , fn ServerFillBuffer (uExtent : REFGUID , riid : REFIID , pDataSize : * mut ULONG , pDataBuffer : * mut c_void , hrFault : HRESULT ,) -> () , } }
};
}
