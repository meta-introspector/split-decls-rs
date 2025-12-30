// Generated macro for macro_26336 (macro)
macro_rules! Depcrate_um_d3d12macro_26336 {
() => {
// Module: crate::um::d3d12
// Provides: {"macro_26336"}
// Dependencies: {}
RIDL ! { # [uuid (0xc4fec28f , 0x7966 , 0x4e95 , 0x9f , 0x94 , 0xf4 , 0x31 , 0xcb , 0x56 , 0xc3 , 0xb8)] interface ID3D12Object (ID3D12ObjectVtbl) : IUnknown (IUnknownVtbl) { fn GetPrivateData (guid : REFGUID , pDataSize : * mut UINT , pData : * mut c_void ,) -> HRESULT , fn SetPrivateData (guid : REFGUID , DataSize : UINT , pData : * const c_void ,) -> HRESULT , fn SetPrivateDataInterface (guid : REFGUID , pData : * const IUnknown ,) -> HRESULT , fn SetName (Name : LPCWSTR ,) -> HRESULT , } }
};
}
