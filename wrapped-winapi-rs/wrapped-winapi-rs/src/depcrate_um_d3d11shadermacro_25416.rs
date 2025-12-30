// Generated macro for macro_25416 (macro)
macro_rules! Depcrate_um_d3d11shadermacro_25416 {
() => {
// Module: crate::um::d3d11shader
// Provides: {"macro_25416"}
// Dependencies: {}
RIDL ! { # [uuid (0x207bcecb , 0xd683 , 0x4a06 , 0xa8 , 0xa3 , 0x9b , 0x14 , 0x9b , 0x9f , 0x73 , 0xa4)] interface ID3D11FunctionReflection (ID3D11FunctionReflectionVtbl) { fn GetDesc (pDesc : * mut D3D11_FUNCTION_DESC ,) -> HRESULT , fn GetConstantBufferByIndex (BufferIndex : UINT ,) -> * mut ID3D11ShaderReflectionConstantBuffer , fn GetConstantBufferByName (Name : LPCSTR ,) -> * mut ID3D11ShaderReflectionConstantBuffer , fn GetResourceBindingDesc (ResourceIndex : UINT , pDesc : * mut D3D11_SHADER_INPUT_BIND_DESC ,) -> HRESULT , fn GetVariableByName (Name : LPCSTR ,) -> * mut ID3D11ShaderReflectionVariable , fn GetResourceBindingDescByName (Name : LPCSTR , pDesc : * mut D3D11_SHADER_INPUT_BIND_DESC ,) -> HRESULT , fn GetFunctionParameter (ParameterIndex : INT ,) -> * mut ID3D11FunctionParameterReflection , } }
};
}
