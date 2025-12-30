// Generated macro for macro_26456 (macro)
macro_rules! Depcrate_um_d3d12shadermacro_26456 {
() => {
// Module: crate::um::d3d12shader
// Provides: {"macro_26456"}
// Dependencies: {}
RIDL ! { # [uuid (0x8337a8a6 , 0xa216 , 0x444a , 0xb2 , 0xf4 , 0x31 , 0x47 , 0x33 , 0xa7 , 0x3a , 0xea)] interface ID3D12ShaderReflectionVariable (ID3D12ShaderReflectionVariableVtbl) { fn GetDesc (pDesc : * mut D3D12_SHADER_VARIABLE_DESC ,) -> HRESULT , fn GetType () -> * mut ID3D12ShaderReflectionType , fn GetBuffer () -> * mut ID3D12ShaderReflectionConstantBuffer , fn GetInterfaceSlot (uArrayIndex : UINT ,) -> UINT , } }
};
}
