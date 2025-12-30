// Generated macro for macro_40515 (macro)
macro_rules! Depcrate_um_wincodecmacro_40515 {
() => {
// Module: crate::um::wincodec
// Provides: {"macro_40515"}
// Dependencies: {}
RIDL ! { # [uuid (0x3d4c0c61 , 0x18a4 , 0x41e4 , 0xbd , 0x80 , 0x48 , 0x1a , 0x4f , 0xc9 , 0xf4 , 0x64)] interface IWICDdsFrameDecode (IWICDdsFrameDecodeVtbl) : IUnknown (IUnknownVtbl) { fn GetSizeInBlocks (pWidthInBlocks : * mut UINT , pHeightInBlocks : * mut UINT ,) -> HRESULT , fn GetFormatInfo (pFormatInfo : * mut WICDdsFormatInfo ,) -> HRESULT , fn CopyBlocks (prcBoundsInBlocks : * const WICRect , cbStride : UINT , cbBufferSize : UINT , pbBuffer : * mut BYTE ,) -> HRESULT , } }
};
}
