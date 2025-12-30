// Generated macro for macro_3436 (macro)
macro_rules! Depcrate_shared_dxgi1_3macro_3436 {
() => {
// Module: crate::shared::dxgi1_3
// Provides: {"macro_3436"}
// Dependencies: {}
RIDL ! { # [uuid (0xa8be2ac4 , 0x199f , 0x4946 , 0xb3 , 0x31 , 0x79 , 0x59 , 0x9f , 0xb9 , 0x8d , 0xe7)] interface IDXGISwapChain2 (IDXGISwapChain2Vtbl) : IDXGISwapChain1 (IDXGISwapChain1Vtbl) { fn SetSourceSize (Width : UINT , Height : UINT ,) -> HRESULT , fn GetSourceSize (pWidth : * mut UINT , pHeight : * mut UINT ,) -> HRESULT , fn SetMaximumFrameLatency (MaxLatency : UINT ,) -> HRESULT , fn GetMaximumFrameLatency (pMaxLatency : * mut UINT ,) -> HRESULT , fn GetFrameLatencyWaitableObject () -> HANDLE , fn SetMatrixTransform (pMatrix : * const DXGI_MATRIX_3X2_F ,) -> HRESULT , fn GetMatrixTransform (pMatrix : * mut DXGI_MATRIX_3X2_F ,) -> HRESULT , } }
};
}
