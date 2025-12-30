// Generated macro for macro_40444 (macro)
macro_rules! Depcrate_um_wincodecmacro_40444 {
() => {
// Module: crate::um::wincodec
// Provides: {"macro_40444"}
// Dependencies: {}
RIDL ! { # [uuid (0x135ff860 , 0x22b7 , 0x4ddf , 0xb0 , 0xf6 , 0x21 , 0x8f , 0x4f , 0x29 , 0x9a , 0x43)] interface IWICStream (IWICStreamVtbl) : IStream (IStreamVtbl) { fn InitializeFromIStream (pIStream : * const IStream ,) -> HRESULT , fn InitializeFromFilename (wzFileName : LPCWSTR , dwDesiredAccess : DWORD ,) -> HRESULT , fn InitializeFromMemory (pbBuffer : WICInProcPointer , cbBufferSize : DWORD ,) -> HRESULT , fn InitializeFromIStreamRegion (pIStream : * const IStream , ulOffset : ULARGE_INTEGER , ulMaxSize : ULARGE_INTEGER ,) -> HRESULT , } }
};
}
