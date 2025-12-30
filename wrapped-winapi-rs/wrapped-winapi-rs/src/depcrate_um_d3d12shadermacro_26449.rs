// Generated macro for macro_26449 (macro)
macro_rules! Depcrate_um_d3d12shadermacro_26449 {
() => {
// Module: crate::um::d3d12shader
// Provides: {"macro_26449"}
// Dependencies: {}
RIDL ! { # [uuid (0x1108795c , 0x2772 , 0x4ba9 , 0xb2 , 0xa8 , 0xd4 , 0x64 , 0xdc , 0x7e , 0x27 , 0x99)] interface ID3D12FunctionReflection (ID3D12FunctionReflectionVtbl) { fn GetDesc (pDesc : * mut D3D12_FUNCTION_DESC ,) -> HRESULT , fn GetConstantBufferByIndex (BufferIndex : UINT ,) -> * mut ID3D12ShaderReflectionConstantBuffer , fn GetConstantBufferByName (Name : LPCSTR ,) -> * mut ID3D12ShaderReflectionConstantBuffer , fn GetResourceBindingDesc (ResourceIndex : UINT , pDesc : * mut D3D12_SHADER_INPUT_BIND_DESC ,) -> HRESULT , fn GetVariableByName (Name : LPCSTR ,) -> * mut ID3D12ShaderReflectionVariable , fn GetResourceBindingDescByName (Name : LPCSTR , pDesc : * mut D3D12_SHADER_INPUT_BIND_DESC ,) -> HRESULT , fn GetFunctionParameter (ParameterIndex : INT ,) -> * mut ID3D12FunctionParameterReflection , } }
};
}
