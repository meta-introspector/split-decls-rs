// Generated macro for macro_28088 (macro)
macro_rules! Depcrate_um_dwrite_3macro_28088 {
() => {
// Module: crate::um::dwrite_3
// Provides: {"macro_28088"}
// Dependencies: {}
RIDL ! { # [uuid (0xb71e6052 , 0x5aea , 0x4fa3 , 0x83 , 0x2e , 0xf6 , 0x0d , 0x43 , 0x1f , 0x7e , 0x91)] interface IDWriteFontDownloadQueue (IDWriteFontDownloadQueueVtbl) : IUnknown (IUnknownVtbl) { fn AddListener (listener : * mut IDWriteFontDownloadListener , token : * mut UINT32 ,) -> HRESULT , fn RemoveListener (token : UINT32 ,) -> HRESULT , fn IsEmpty () -> BOOL , fn BeginDownload (context : * mut IUnknown ,) -> HRESULT , fn CancelDownload () -> HRESULT , fn GetGenerationCount () -> UINT64 , } }
};
}
