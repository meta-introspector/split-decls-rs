// Generated macro for macro_25424 (macro)
macro_rules! Depcrate_um_d3d11shadermacro_25424 {
() => {
// Module: crate::um::d3d11shader
// Provides: {"macro_25424"}
// Dependencies: {}
RIDL ! { # [uuid (0x59a6cd0e , 0xe10d , 0x4c1f , 0x88 , 0xc0 , 0x63 , 0xab , 0xa1 , 0xda , 0xf3 , 0x0e)] interface ID3D11Linker (ID3D11LinkerVtbl) : IUnknown (IUnknownVtbl) { fn Link (pEntry : * mut ID3D11ModuleInstance , pEntryName : LPCSTR , pTargetName : LPCSTR , uFlags : UINT , ppShaderBlob : * mut * mut ID3DBlob , ppErrorBuffer : * mut * mut ID3DBlob ,) -> HRESULT , fn UseLibrary (pLibraryMI : * mut ID3D11ModuleInstance ,) -> HRESULT , fn AddClipPlaneFromCBuffer (uCBufferSlot : UINT , uCBufferEntry : UINT ,) -> HRESULT , } }
};
}
