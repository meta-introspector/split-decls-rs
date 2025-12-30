// Generated macro for macro_3325 (macro)
macro_rules! Depcrate_shared_dxgimacro_3325 {
() => {
// Module: crate::shared::dxgi
// Provides: {"macro_3325"}
// Dependencies: {}
RIDL ! { # [uuid (0xae02eedb , 0xc735 , 0x4690 , 0x8d , 0x52 , 0x5a , 0x8d , 0xc2 , 0x02 , 0x13 , 0xaa)] interface IDXGIOutput (IDXGIOutputVtbl) : IDXGIObject (IDXGIObjectVtbl) { fn GetDesc (pDesc : * mut DXGI_OUTPUT_DESC ,) -> HRESULT , fn GetDisplayModeList (EnumFormat : DXGI_FORMAT , Flags : UINT , pNumModes : * mut UINT , pDesc : * mut DXGI_MODE_DESC ,) -> HRESULT , fn FindClosestMatchingMode (pModeToMatch : * const DXGI_MODE_DESC , pClosestMatch : * mut DXGI_MODE_DESC , pConcernedDevice : * mut IUnknown ,) -> HRESULT , fn WaitForVBlank () -> HRESULT , fn TakeOwnership (pDevice : * mut IUnknown , Exclusive : BOOL ,) -> HRESULT , fn ReleaseOwnership () -> () , fn GetGammaControlCapabilities (pGammaCaps : * mut DXGI_GAMMA_CONTROL_CAPABILITIES ,) -> HRESULT , fn SetGammaControl (pArray : * const DXGI_GAMMA_CONTROL ,) -> HRESULT , fn GetGammaControl (pArray : * mut DXGI_GAMMA_CONTROL ,) -> HRESULT , fn SetDisplaySurface (pScanoutSurface : * mut IDXGISurface ,) -> HRESULT , fn GetDisplaySurfaceData (pDestination : * mut IDXGISurface ,) -> HRESULT , fn GetFrameStatistics (pStats : * mut DXGI_FRAME_STATISTICS ,) -> HRESULT , } }
};
}
