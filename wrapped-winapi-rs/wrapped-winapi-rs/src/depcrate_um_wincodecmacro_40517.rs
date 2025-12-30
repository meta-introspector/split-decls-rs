// Generated macro for macro_40517 (macro)
macro_rules! Depcrate_um_wincodecmacro_40517 {
() => {
// Module: crate::um::wincodec
// Provides: {"macro_40517"}
// Dependencies: {}
RIDL ! { # [uuid (0x2f0c601f , 0xd2c6 , 0x468c , 0xab , 0xfa , 0x49 , 0x49 , 0x5d , 0x98 , 0x3e , 0xd1)] interface IWICJpegFrameEncode (IWICJpegFrameEncodeVtbl) : IUnknown (IUnknownVtbl) { fn GetAcHuffmanTable (scanIndex : UINT , tableIndex : UINT , pAcHuffmanTable : * mut DXGI_JPEG_AC_HUFFMAN_TABLE ,) -> HRESULT , fn GetDcHuffmanTable (scanIndex : UINT , tableIndex : UINT , pDcHuffmanTable : * mut DXGI_JPEG_DC_HUFFMAN_TABLE ,) -> HRESULT , fn GetQuantizationTable (scanIndex : UINT , tableIndex : UINT , pQuantizationTable : * mut DXGI_JPEG_QUANTIZATION_TABLE ,) -> HRESULT , fn WriteScan (cbScanData : UINT , pbScanData : * const BYTE ,) -> HRESULT , } }
};
}
