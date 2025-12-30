// Generated macro for macro_33863 (macro)
macro_rules! Depcrate_um_objidlbasemacro_33863 {
() => {
// Module: crate::um::objidlbase
// Provides: {"macro_33863"}
// Dependencies: {}
RIDL ! { # [uuid (0x00000003 , 0x0000 , 0x0000 , 0xc0 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x46)] interface IMarshal (IMarshalVtbl) : IUnknown (IUnknownVtbl) { fn GetUnmarshalClass (riid : REFIID , pv : * mut c_void , dwDestContext : DWORD , pvDestContext : * mut c_void , mshlflags : DWORD , pCid : * mut CLSID ,) -> HRESULT , fn GetMarshalSizeMax (riid : REFIID , pv : * mut c_void , dwDestContext : DWORD , pvDestContext : * mut c_void , mshlflags : DWORD , pSize : * mut DWORD ,) -> HRESULT , fn MarshalInterface (pStm : * mut IStream , riid : REFIID , pv : * mut c_void , dwDestContext : DWORD , pvDestContext : * mut c_void , mshlflags : DWORD ,) -> HRESULT , fn UnmarshalInterface (pStm : * mut IStream , riid : REFIID , ppv : * mut * mut c_void ,) -> HRESULT , fn ReleaseMarshalData (pStm : * mut IStream ,) -> HRESULT , fn DisconnectObject (dwReserved : DWORD ,) -> HRESULT , } }
};
}
