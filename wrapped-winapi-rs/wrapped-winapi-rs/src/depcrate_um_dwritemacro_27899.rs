// Generated macro for macro_27899 (macro)
macro_rules! Depcrate_um_dwritemacro_27899 {
() => {
// Module: crate::um::dwrite
// Provides: {"macro_27899"}
// Dependencies: {}
RIDL ! { # [uuid (0x6d4865fe , 0x0ab8 , 0x4d91 , 0x8f , 0x62 , 0x5d , 0xd6 , 0xbe , 0x34 , 0xa3 , 0xe0)] interface IDWriteFontFileStream (IDWriteFontFileStreamVtbl) : IUnknown (IUnknownVtbl) { fn ReadFileFragment (fragmentStart : * mut * const c_void , fileOffset : UINT64 , fragmentSize : UINT64 , fragmentContext : * mut * mut c_void ,) -> HRESULT , fn ReleaseFileFragment (fragmentContext : * mut c_void ,) -> () , fn GetFileSize (fileSize : * mut UINT64 ,) -> HRESULT , fn GetLastWriteTime (lastWriteTime : * mut UINT64 ,) -> HRESULT , } }
};
}
