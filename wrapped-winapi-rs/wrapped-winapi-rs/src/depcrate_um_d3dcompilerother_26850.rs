// Generated macro for other_26850 (other)
macro_rules! Depcrate_um_d3dcompilerother_26850 {
() => {
// Module: crate::um::d3dcompiler
// Provides: {"other_26850"}
// Dependencies: {}
extern "system" { pub fn D3DCompressShaders (uNumShaders : UINT , pShaderData : * mut D3D_SHADER_DATA , uFlags : UINT , ppCompressedData : * mut * mut ID3DBlob ,) -> HRESULT ; pub fn D3DDecompressShaders (pSrcData : LPCVOID , SrcDataSize : SIZE_T , uNumShaders : UINT , uStartIndex : UINT , pIndices : * mut UINT , uFlags : UINT , ppShaders : * mut * mut ID3DBlob , pTotalShaders : * mut UINT ,) -> HRESULT ; }
};
}
