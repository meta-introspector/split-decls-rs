// Generated macro for macro_33830 (macro)
macro_rules! Depcrate_um_objidlmacro_33830 {
() => {
// Module: crate::um::objidl
// Provides: {"macro_33830"}
// Dependencies: {}
RIDL ! { # [uuid (0x00000109 , 0x0000 , 0x0000 , 0xc0 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x46)] interface IPersistStream (IPersistStreamVtbl) : IPersist (IPersistVtbl) { fn IsDirty () -> HRESULT , fn Load (pStm : * mut IStream ,) -> HRESULT , fn Save (pStm : * mut IStream , fClearDirty : BOOL ,) -> HRESULT , fn GetSizeMax (pcbSize : * mut ULARGE_INTEGER ,) -> HRESULT , } }
};
}
