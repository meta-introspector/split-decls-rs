// Generated macro for macro_3481 (macro)
macro_rules! Depcrate_shared_dxgi1_5macro_3481 {
() => {
// Module: crate::shared::dxgi1_5
// Provides: {"macro_3481"}
// Dependencies: {}
RIDL ! { # [uuid (0x80a07424 , 0xab52 , 0x42eb , 0x83 , 0x3c , 0x0c , 0x42 , 0xfd , 0x28 , 0x2d , 0x98)] interface IDXGIOutput5 (IDXGIOutput5Vtbl) : IDXGIOutput4 (IDXGIOutput4Vtbl) { fn DuplicateOutput1 (pDevice : * mut IUnknown , Flags : UINT , SupportedFormatsCount : UINT , pSupportedFormats : * const DXGI_FORMAT , ppOutputDuplication : * mut * mut IDXGIOutputDuplication ,) -> HRESULT , } }
};
}
