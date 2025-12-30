// Generated macro for macro_40465 (macro)
macro_rules! Depcrate_um_wincodecmacro_40465 {
() => {
// Module: crate::um::wincodec
// Provides: {"macro_40465"}
// Dependencies: {}
RIDL ! { # [uuid (0xe8eda601 , 0x3d48 , 0x431a , 0xab , 0x44 , 0x69 , 0x05 , 0x9b , 0xe8 , 0x8b , 0xbe)] interface IWICPixelFormatInfo (IWICPixelFormatInfoVtbl) : IWICComponentInfo (IWICComponentInfoVtbl) { fn GetFormatGUID (pFormat : * mut GUID ,) -> HRESULT , fn GetColorContext (ppIColorContext : * mut * mut IWICColorContext ,) -> HRESULT , fn GetBitsPerPixel (puiBitsPerPixel : * mut UINT ,) -> HRESULT , fn GetChannelCount (puiChannelCount : * mut UINT ,) -> HRESULT , fn GetChannelMask (uiChannelIndex : UINT , cbMaskBuffer : UINT , pbMaskBuffer : * mut BYTE , pcbActual : * mut UINT ,) -> HRESULT , } }
};
}
