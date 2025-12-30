// Generated macro for macro_25009 (macro)
macro_rules! Depcrate_um_d3d11macro_25009 {
() => {
// Module: crate::um::d3d11
// Provides: {"macro_25009"}
// Dependencies: {}
RIDL ! { # [uuid (0xddf57cba , 0x9543 , 0x46e4 , 0xa1 , 0x2b , 0xf2 , 0x07 , 0xa0 , 0xfe , 0x7f , 0xed)] interface ID3D11ClassLinkage (ID3D11ClassLinkageVtbl) : ID3D11DeviceChild (ID3D11DeviceChildVtbl) { fn GetClassInstance (GetClassInstance : LPCSTR , InstanceIndex : UINT , ppInstance : * mut * mut ID3D11ClassInstance ,) -> HRESULT , fn CreateClassInstance (pClassTypeName : LPCSTR , ConstantBufferOffset : UINT , ConstantVectorOffset : UINT , TextureOffset : UINT , SamplerOffset : UINT , ppInstance : * mut * mut ID3D11ClassInstance ,) -> HRESULT , } }
};
}
