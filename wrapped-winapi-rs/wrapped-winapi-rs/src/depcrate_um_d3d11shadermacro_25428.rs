// Generated macro for macro_25428 (macro)
macro_rules! Depcrate_um_d3d11shadermacro_25428 {
() => {
// Module: crate::um::d3d11shader
// Provides: {"macro_25428"}
// Dependencies: {}
RIDL ! { # [uuid (0x54133220 , 0x1ce8 , 0x43d3 , 0x82 , 0x36 , 0x98 , 0x55 , 0xc5 , 0xce , 0xec , 0xff)] interface ID3D11FunctionLinkingGraph (ID3D11FunctionLinkingGraphVtbl) : IUnknown (IUnknownVtbl) { fn CreateModuleInstance (ppModuleInstance : * mut * mut ID3D11ModuleInstance , ppErrorBuffer : * mut * mut ID3DBlob ,) -> HRESULT , fn SetInputSignature (pInputParameters : * const D3D11_PARAMETER_DESC , cInputParameters : UINT , ppInputNode : * mut * mut ID3D11LinkingNode ,) -> HRESULT , fn SetOutputSignature (pOutputParameters : * const D3D11_PARAMETER_DESC , cOutputParameters : UINT , ppOutputNode : * mut * mut ID3D11LinkingNode ,) -> HRESULT , fn CallFunction (pModuleInstanceNamespace : LPCSTR , pModuleWithFunctionPrototype : * mut ID3D11Module , pFunctionName : LPCSTR , ppCallNode : * mut * mut ID3D11LinkingNode ,) -> HRESULT , fn PassValue (pSrcNode : * mut ID3D11LinkingNode , SrcParameterIndex : INT , pDstNode : * mut ID3D11LinkingNode , DstParameterIndex : INT ,) -> HRESULT , fn PassValueWithSwizzle (pSrcNode : * mut ID3D11LinkingNode , SrcParameterIndex : INT , pSrcSwizzle : LPCSTR , pDstNode : * mut ID3D11LinkingNode , DstParameterIndex : INT , pDstSwizzle : LPCSTR ,) -> HRESULT , fn GetLastError (ppErrorBuffer : * mut * mut ID3DBlob ,) -> HRESULT , fn GenerateHlsl (uFlags : UINT , ppBuffer : * mut * mut ID3DBlob ,) -> HRESULT , } }
};
}
