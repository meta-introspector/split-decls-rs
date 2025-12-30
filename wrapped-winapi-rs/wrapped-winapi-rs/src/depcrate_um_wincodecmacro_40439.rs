// Generated macro for macro_40439 (macro)
macro_rules! Depcrate_um_wincodecmacro_40439 {
() => {
// Module: crate::um::wincodec
// Provides: {"macro_40439"}
// Dependencies: {}
RIDL ! { # [uuid (0x00000123 , 0xa8f2 , 0x4877 , 0xba , 0x0a , 0xfd , 0x2b , 0x66 , 0x45 , 0xfb , 0x94)] interface IWICBitmapLock (IWICBitmapLockVtbl) : IUnknown (IUnknownVtbl) { fn GetSize (puiWidth : * mut UINT , puiHeight : * mut UINT ,) -> HRESULT , fn GetStride (pcbStride : * mut UINT ,) -> HRESULT , fn GetDataPointer (pcbBufferSize : * mut UINT , ppbData : * mut WICInProcPointer ,) -> HRESULT , fn GetPixelFormat (pPixelFormat : * mut WICPixelFormatGUID ,) -> HRESULT , } }
};
}
