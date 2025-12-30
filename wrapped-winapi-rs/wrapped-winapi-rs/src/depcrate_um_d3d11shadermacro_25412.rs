// Generated macro for macro_25412 (macro)
macro_rules! Depcrate_um_d3d11shadermacro_25412 {
() => {
// Module: crate::um::d3d11shader
// Provides: {"macro_25412"}
// Dependencies: {}
RIDL ! { # [uuid (0x8d536ca1 , 0x0cca , 0x4956 , 0xa8 , 0x37 , 0x78 , 0x69 , 0x63 , 0x75 , 0x55 , 0x84)] interface ID3D11ShaderReflection (ID3D11ShaderReflectionVtbl) : IUnknown (IUnknownVtbl) { fn GetDesc (pDesc : * mut D3D11_SHADER_DESC ,) -> HRESULT , fn GetConstantBufferByIndex (Index : UINT ,) -> * mut ID3D11ShaderReflectionConstantBuffer , fn GetConstantBufferByName (Name : LPCSTR ,) -> * mut ID3D11ShaderReflectionConstantBuffer , fn GetResourceBindingDesc (ResourceIndex : UINT , pDesc : * mut D3D11_SHADER_INPUT_BIND_DESC ,) -> HRESULT , fn GetInputParameterDesc (ParameterIndex : UINT , pDesc : * mut D3D11_SIGNATURE_PARAMETER_DESC ,) -> HRESULT , fn GetOutputParameterDesc (ParameterIndex : UINT , pDesc : * mut D3D11_SIGNATURE_PARAMETER_DESC ,) -> HRESULT , fn GetPatchConstantParameterDesc (ParameterIndex : UINT , pDesc : * mut D3D11_SIGNATURE_PARAMETER_DESC ,) -> HRESULT , fn GetVariableByName (Name : LPCSTR ,) -> * mut ID3D11ShaderReflectionVariable , fn GetResourceBindingDescByName (Name : LPCSTR , pDesc : * mut D3D11_SHADER_INPUT_BIND_DESC ,) -> HRESULT , fn GetMovInstructionCount () -> UINT , fn GetMovcInstructionCount () -> UINT , fn GetConversionInstructionCount () -> UINT , fn GetBitwiseInstructionCount () -> UINT , fn GetGSInputPrimitive () -> D3D_PRIMITIVE , fn IsSampleFrequencyShader () -> BOOL , fn GetNumInterfaceSlots () -> UINT , fn GetMinFeatureLevel (pLevel : * mut D3D_FEATURE_LEVEL ,) -> HRESULT , fn GetThreadGroupSize (pSizeX : * mut UINT , pSizeY : * mut UINT , pSizeZ : * mut UINT ,) -> UINT , fn GetRequiresFlags () -> UINT64 , } }
};
}
