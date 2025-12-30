// Generated macro for macro_24889 (macro)
macro_rules! Depcrate_um_d3d11macro_24889 {
() => {
// Module: crate::um::d3d11
// Provides: {"macro_24889"}
// Dependencies: {}
RIDL ! { # [uuid (0x1841e5c8 , 0x16b0 , 0x489b , 0xbc , 0xc8 , 0x44 , 0xcf , 0xb0 , 0xd5 , 0xde , 0xae)] interface ID3D11DeviceChild (ID3D11DeviceChildVtbl) : IUnknown (IUnknownVtbl) { fn GetDevice (ppDevice : * mut * mut ID3D11Device ,) -> () , fn GetPrivateData (guid : REFGUID , pDataSize : * mut UINT , pData : * mut c_void ,) -> HRESULT , fn SetPrivateData (guid : REFGUID , DataSize : UINT , pData : * const c_void ,) -> HRESULT , fn SetPrivateDataInterface (guid : REFGUID , pData : * const IUnknown ,) -> HRESULT , } }
};
}
