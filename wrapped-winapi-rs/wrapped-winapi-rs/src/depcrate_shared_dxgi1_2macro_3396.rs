// Generated macro for macro_3396 (macro)
macro_rules! Depcrate_shared_dxgi1_2macro_3396 {
() => {
// Module: crate::shared::dxgi1_2
// Provides: {"macro_3396"}
// Dependencies: {}
RIDL ! { # [uuid (0x30961379 , 0x4609 , 0x4a41 , 0x99 , 0x8e , 0x54 , 0xfe , 0x56 , 0x7e , 0xe0 , 0xc1)] interface IDXGIResource1 (IDXGIResource1Vtbl) : IDXGIResource (IDXGIResourceVtbl) { fn CreateSubresourceSurface (index : UINT , ppSurface : * mut * mut IDXGISurface2 ,) -> HRESULT , fn CreateSharedHandle (pAttributes : * const SECURITY_ATTRIBUTES , dwAccess : DWORD , lpName : LPCWSTR , pHandle : * mut HANDLE ,) -> HRESULT , } }
};
}
