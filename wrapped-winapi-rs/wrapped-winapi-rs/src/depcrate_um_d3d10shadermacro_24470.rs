// Generated macro for macro_24470 (macro)
macro_rules! Depcrate_um_d3d10shadermacro_24470 {
() => {
// Module: crate::um::d3d10shader
// Provides: {"macro_24470"}
// Dependencies: {}
RIDL ! { # [uuid (0xc530ad7d , 0x9b16 , 0x4395 , 0xa9 , 0x79 , 0xba , 0x2e , 0xcf , 0xf8 , 0x3a , 0xdd)] interface ID3D10ShaderReflectionType (ID3D10ShaderReflectionTypeVtbl) { fn GetDesc (pDesc : * mut D3D10_SHADER_TYPE_DESC ,) -> HRESULT , fn GetMemberTypeByIndex (Index : UINT ,) -> * mut ID3D10ShaderReflectionType , fn GetMemberTypeByName (Name : LPCSTR ,) -> * mut ID3D10ShaderReflectionType , fn GetMemberTypeName (Index : UINT ,) -> LPCSTR , } }
};
}
