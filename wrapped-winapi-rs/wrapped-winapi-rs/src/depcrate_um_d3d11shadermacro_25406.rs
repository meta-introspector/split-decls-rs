// Generated macro for macro_25406 (macro)
macro_rules! Depcrate_um_d3d11shadermacro_25406 {
() => {
// Module: crate::um::d3d11shader
// Provides: {"macro_25406"}
// Dependencies: {}
RIDL ! { # [uuid (0x6e6ffa6a , 0x9bae , 0x4613 , 0xa5 , 0x1e , 0x91 , 0x65 , 0x2d , 0x50 , 0x8c , 0x21)] interface ID3D11ShaderReflectionType (ID3D11ShaderReflectionTypeVtbl) { fn GetDesc (pDesc : * mut D3D11_SHADER_TYPE_DESC ,) -> HRESULT , fn GetMemberTypeByIndex (Index : UINT ,) -> * mut ID3D11ShaderReflectionType , fn GetMemberTypeByName (Name : LPCSTR ,) -> * mut ID3D11ShaderReflectionType , fn GetMemberTypeName (Index : UINT ,) -> LPCSTR , fn IsEqual (pType : * mut ID3D11ShaderReflectionType ,) -> HRESULT , fn GetSubType () -> * mut ID3D11ShaderReflectionType , fn GetBaseClass () -> * mut ID3D11ShaderReflectionType , fn GetNumInterfaces () -> UINT , fn GetInterfaceByIndex (uIndex : UINT ,) -> * mut ID3D11ShaderReflectionType , fn IsOfType (pType : * mut ID3D11ShaderReflectionType ,) -> HRESULT , fn ImplementsInterface (pBase : * mut ID3D11ShaderReflectionType ,) -> HRESULT , } }
};
}
