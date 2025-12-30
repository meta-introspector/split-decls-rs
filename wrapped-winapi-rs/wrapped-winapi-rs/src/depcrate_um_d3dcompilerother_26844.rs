// Generated macro for other_26844 (other)
macro_rules! Depcrate_um_d3dcompilerother_26844 {
() => {
// Module: crate::um::d3dcompiler
// Provides: {"other_26844"}
// Dependencies: {}
extern "system" { pub fn D3DGetTraceInstructionOffsets (pSrcData : LPCVOID , SrcDataSize : SIZE_T , Flags : UINT , StartInstIndex : SIZE_T , NumInsts : SIZE_T , pOffsets : * mut SIZE_T , pTotalInsts : * mut SIZE_T ,) -> HRESULT ; pub fn D3DGetInputSignatureBlob (pSrcData : LPCVOID , SrcDataSize : SIZE_T , ppSignatureBlob : * mut * mut ID3DBlob ,) -> HRESULT ; pub fn D3DGetOutputSignatureBlob (pSrcData : LPCVOID , SrcDataSize : SIZE_T , ppSignatureBlob : * mut * mut ID3DBlob ,) -> HRESULT ; pub fn D3DGetInputAndOutputSignatureBlob (pSrcData : LPCVOID , SrcDataSize : SIZE_T , ppSignatureBlob : * mut * mut ID3DBlob ,) -> HRESULT ; }
};
}
