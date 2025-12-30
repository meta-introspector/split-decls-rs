// Generated macro for macro_3394 (macro)
macro_rules! Depcrate_shared_dxgi1_2macro_3394 {
() => {
// Module: crate::shared::dxgi1_2
// Provides: {"macro_3394"}
// Dependencies: {}
RIDL ! { # [uuid (0x00cddea8 , 0x939b , 0x4b83 , 0xa3 , 0x40 , 0xa6 , 0x85 , 0x22 , 0x66 , 0x66 , 0xcc)] interface IDXGIOutput1 (IDXGIOutput1Vtbl) : IDXGIOutput (IDXGIOutputVtbl) { fn GetDisplayModeList1 (EnumFormat : DXGI_FORMAT , Flags : UINT , pNumModes : * mut UINT , pDesc : * mut DXGI_MODE_DESC1 ,) -> HRESULT , fn FindClosestMatchingMode1 (pModeToMatch : * const DXGI_MODE_DESC1 , pClosestMatch : * mut DXGI_MODE_DESC1 , pConcernedDevice : * mut IUnknown ,) -> HRESULT , fn GetDisplaySurfaceData1 (pDestination : * mut IDXGIResource ,) -> HRESULT , fn DuplicateOutput (pDevice : * mut IUnknown , ppOutputDuplication : * mut * mut IDXGIOutputDuplication ,) -> HRESULT , } }
};
}
