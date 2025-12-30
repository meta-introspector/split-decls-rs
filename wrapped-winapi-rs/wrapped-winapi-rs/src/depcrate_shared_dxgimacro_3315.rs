// Generated macro for macro_3315 (macro)
macro_rules! Depcrate_shared_dxgimacro_3315 {
() => {
// Module: crate::shared::dxgi
// Provides: {"macro_3315"}
// Dependencies: {}
RIDL ! { # [uuid (0xaec22fb8 , 0x76f3 , 0x4639 , 0x9b , 0xe0 , 0x28 , 0xeb , 0x43 , 0xa6 , 0x7a , 0x2e)] interface IDXGIObject (IDXGIObjectVtbl) : IUnknown (IUnknownVtbl) { fn SetPrivateData (Name : REFGUID , DataSize : UINT , pData : * const c_void ,) -> HRESULT , fn SetPrivateDataInterface (Name : REFGUID , pUnknown : * const IUnknown ,) -> HRESULT , fn GetPrivateData (Name : REFGUID , pDataSize : * mut UINT , pData : * mut c_void ,) -> HRESULT , fn GetParent (riid : REFIID , ppParent : * mut * mut c_void ,) -> HRESULT , } }
};
}
