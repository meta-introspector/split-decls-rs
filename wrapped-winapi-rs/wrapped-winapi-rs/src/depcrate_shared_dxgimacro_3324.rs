// Generated macro for macro_3324 (macro)
macro_rules! Depcrate_shared_dxgimacro_3324 {
() => {
// Module: crate::shared::dxgi
// Provides: {"macro_3324"}
// Dependencies: {}
RIDL ! { # [uuid (0x2411e7e1 , 0x12ac , 0x4ccf , 0xbd , 0x14 , 0x97 , 0x98 , 0xe8 , 0x53 , 0x4d , 0xc0)] interface IDXGIAdapter (IDXGIAdapterVtbl) : IDXGIObject (IDXGIObjectVtbl) { fn EnumOutputs (Output : UINT , ppOutput : * mut * mut IDXGIOutput ,) -> HRESULT , fn GetDesc (pDesc : * mut DXGI_ADAPTER_DESC ,) -> HRESULT , fn CheckInterfaceSupport (InterfaceName : REFGUID , pUMDVersion : * mut LARGE_INTEGER ,) -> HRESULT , } }
};
}
