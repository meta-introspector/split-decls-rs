// Generated macro for macro_26454 (macro)
macro_rules! Depcrate_um_d3d12shadermacro_26454 {
() => {
// Module: crate::um::d3d12shader
// Provides: {"macro_26454"}
// Dependencies: {}
RIDL ! { # [uuid (0xe913c351 , 0x783d , 0x48ca , 0xa1 , 0xd1 , 0x4f , 0x30 , 0x62 , 0x84 , 0xad , 0x56)] interface ID3D12ShaderReflectionType (ID3D12ShaderReflectionTypeVtbl) { fn GetDesc (pDesc : * mut D3D12_SHADER_TYPE_DESC ,) -> HRESULT , fn GetMemberTypeByIndex (Index : UINT ,) -> * mut ID3D12ShaderReflectionType , fn GetMemberTypeByName (Name : LPCSTR ,) -> * mut ID3D12ShaderReflectionType , fn GetMemberTypeName (Index : UINT ,) -> LPCSTR , fn IsEqual (pType : * mut ID3D12ShaderReflectionType ,) -> HRESULT , fn GetSubType () -> * mut ID3D12ShaderReflectionType , fn GetBaseClass () -> * mut ID3D12ShaderReflectionType , fn GetNumInterfaces () -> UINT , fn GetInterfaceByIndex (uIndex : UINT ,) -> * mut ID3D12ShaderReflectionType , fn IsOfType (pType : * mut ID3D12ShaderReflectionType ,) -> HRESULT , fn ImplementsInterface (pBase : * mut ID3D12ShaderReflectionType ,) -> HRESULT , } }
};
}
