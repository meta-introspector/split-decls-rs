// Generated macro for macro_40516 (macro)
macro_rules! Depcrate_um_wincodecmacro_40516 {
() => {
// Module: crate::um::wincodec
// Provides: {"macro_40516"}
// Dependencies: {}
RIDL ! { # [uuid (0x8939f66e , 0xc46a , 0x4c21 , 0xa9 , 0xd1 , 0x98 , 0xb3 , 0x27 , 0xce , 0x16 , 0x79)] interface IWICJpegFrameDecode (IWICJpegFrameDecodeVtbl) : IUnknown (IUnknownVtbl) { fn DoesSupportIndexing (pfIndexingSupported : * mut BOOL ,) -> HRESULT , fn SetIndexing (options : WICJpegIndexingOptions , horizontalIntervalSize : UINT ,) -> HRESULT , fn ClearIndexing () -> HRESULT , fn GetAcHuffmanTable (scanIndex : UINT , tableIndex : UINT , pAcHuffmanTable : * mut DXGI_JPEG_AC_HUFFMAN_TABLE ,) -> HRESULT , fn GetDcHuffmanTable (scanIndex : UINT , tableIndex : UINT , pDcHuffmanTable : * mut DXGI_JPEG_DC_HUFFMAN_TABLE ,) -> HRESULT , fn GetQuantizationTable (scanIndex : UINT , tableIndex : UINT , pQuantizationTable : * mut DXGI_JPEG_QUANTIZATION_TABLE ,) -> HRESULT , fn GetFrameHeader (pFrameHeader : * mut WICJpegFrameHeader ,) -> HRESULT , fn GetScanHeader (scanIndex : UINT , pScanHeader : * mut WICJpegScanHeader ,) -> HRESULT , fn CopyScan (scanIndex : UINT , scanOffset : UINT , cbScanData : UINT , pbScanData : * mut BYTE , pcbScanDataActual : * mut UINT ,) -> HRESULT , fn CopyMinimalStream (streamOffset : UINT , cbStreamData : UINT , pbStreamData : * mut BYTE , pcbStreamDataActual : * mut UINT ,) -> HRESULT , } }
};
}
