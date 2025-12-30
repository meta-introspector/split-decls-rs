// Generated macro for macro_40440 (macro)
macro_rules! Depcrate_um_wincodecmacro_40440 {
() => {
// Module: crate::um::wincodec
// Provides: {"macro_40440"}
// Dependencies: {}
RIDL ! { # [uuid (0x00000121 , 0xa8f2 , 0x4877 , 0xba , 0x0a , 0xfd , 0x2b , 0x66 , 0x45 , 0xfb , 0x94)] interface IWICBitmap (IWICBitmapVtbl) : IWICBitmapSource (IWICBitmapSourceVtbl) { fn Lock (prcLock : * const WICRect , flags : DWORD , ppILock : * mut * mut IWICBitmapLock ,) -> HRESULT , fn SetPalette (pIPalette : * const IWICPalette ,) -> HRESULT , fn SetResolution (dpiX : c_double , dpiY : c_double ,) -> HRESULT , } }
};
}
