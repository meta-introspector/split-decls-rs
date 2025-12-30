// Generated macro for macro_24479 (macro)
macro_rules! Depcrate_um_d3d10shadermacro_24479 {
() => {
// Module: crate::um::d3d10shader
// Provides: {"macro_24479"}
// Dependencies: {}
RIDL ! { # [uuid (0xd40e20b6 , 0xf8f7 , 0x42ad , 0xab , 0x20 , 0x4b , 0xaf , 0x8f , 0x15 , 0xdf , 0xaa)] interface ID3D10ShaderReflection (ID3D10ShaderReflectionVtbl) : IUnknown (IUnknownVtbl) { fn GetDesc (pDesc : * mut D3D10_SHADER_DESC ,) -> HRESULT , fn GetConstantBufferByIndex (Index : UINT ,) -> * mut ID3D10ShaderReflectionConstantBuffer , fn GetConstantBufferByName (Name : LPCSTR ,) -> * mut ID3D10ShaderReflectionConstantBuffer , fn GetResourceBindingDesc (ResourceIndex : UINT , pDesc : * mut D3D10_SHADER_INPUT_BIND_DESC ,) -> HRESULT , fn GetInputParameterDesc (ParameterIndex : UINT , pDesc : * mut D3D10_SIGNATURE_PARAMETER_DESC ,) -> HRESULT , fn GetOutputParameterDesc (ParameterIndex : UINT , pDesc : * mut D3D10_SIGNATURE_PARAMETER_DESC ,) -> HRESULT , } }
};
}
