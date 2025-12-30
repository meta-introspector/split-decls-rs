// Generated macro for macro_40451 (macro)
macro_rules! Depcrate_um_wincodecmacro_40451 {
() => {
// Module: crate::um::wincodec
// Provides: {"macro_40451"}
// Dependencies: {}
RIDL ! { # [uuid (0x04c75bf8 , 0x3ce1 , 0x473b , 0xac , 0xc5 , 0x3c , 0xc4 , 0xf5 , 0xe9 , 0x49 , 0x99)] interface IWICImageEncoder (IWICImageEncoderVtbl) : IUnknown (IUnknownVtbl) { fn WriteFrame (pImage : * const ID2D1Image , pFrameEncode : * const IWICBitmapFrameEncode , pImageParameters : * const WICImageParameters ,) -> HRESULT , fn WriteFrameThumbnail (pImage : * const ID2D1Image , pFrameEncode : * const IWICBitmapFrameEncode , pImageParameters : * const WICImageParameters ,) -> HRESULT , fn WriteThumbnail (pImage : * const ID2D1Image , pEncoder : * const IWICBitmapEncoder , pImageParameters : * const WICImageParameters ,) -> HRESULT , } }
};
}
