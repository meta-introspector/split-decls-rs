// Generated macro for macro_26458 (macro)
macro_rules! Depcrate_um_d3d12shadermacro_26458 {
() => {
// Module: crate::um::d3d12shader
// Provides: {"macro_26458"}
// Dependencies: {}
RIDL ! { # [uuid (0x5a58797d , 0xa72c , 0x478d , 0x8b , 0xa2 , 0xef , 0xc6 , 0xb0 , 0xef , 0xe8 , 0x8e)] interface ID3D12ShaderReflection (ID3D12ShaderReflectionVtbl) : IUnknown (IUnknownVtbl) { fn GetDesc (pDesc : * mut D3D12_SHADER_DESC ,) -> HRESULT , fn GetConstantBufferByIndex (Index : UINT ,) -> * mut ID3D12ShaderReflectionConstantBuffer , fn GetConstantBufferByName (Name : LPCSTR ,) -> * mut ID3D12ShaderReflectionConstantBuffer , fn GetResourceBindingDesc (ResourceIndex : UINT , pDesc : * mut D3D12_SHADER_INPUT_BIND_DESC ,) -> HRESULT , fn GetInputParameterDesc (ParameterIndex : UINT , pDesc : * mut D3D12_SIGNATURE_PARAMETER_DESC ,) -> HRESULT , fn GetOutputParameterDesc (ParameterIndex : UINT , pDesc : * mut D3D12_SIGNATURE_PARAMETER_DESC ,) -> HRESULT , fn GetPatchConstantParameterDesc (ParameterIndex : UINT , pDesc : * mut D3D12_SIGNATURE_PARAMETER_DESC ,) -> HRESULT , fn GetVariableByName (Name : LPCSTR ,) -> * mut ID3D12ShaderReflectionVariable , fn GetResourceBindingDescByName (Name : LPCSTR , pDesc : * mut D3D12_SHADER_INPUT_BIND_DESC ,) -> HRESULT , fn GetMovInstructionCount () -> UINT , fn GetMovcInstructionCount () -> UINT , fn GetConversionInstructionCount () -> UINT , fn GetBitwiseInstructionCount () -> UINT , fn GetGSInputPrimitive () -> D3D_PRIMITIVE , fn IsSampleFrequencyShader () -> BOOL , fn GetNumInterfaceSlots () -> UINT , fn GetMinFeatureLevel (pLevel : * mut D3D_FEATURE_LEVEL ,) -> HRESULT , fn GetThreadGroupSize (pSizeX : * mut UINT , pSizeY : * mut UINT , pSizeZ : * mut UINT ,) -> UINT , fn GetRequiresFlags () -> UINT64 , } }
};
}
