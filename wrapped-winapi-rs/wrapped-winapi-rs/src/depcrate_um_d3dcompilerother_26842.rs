// Generated macro for other_26842 (other)
macro_rules! Depcrate_um_d3dcompilerother_26842 {
() => {
// Module: crate::um::d3dcompiler
// Provides: {"other_26842"}
// Dependencies: {}
extern "system" { pub fn D3DDisassemble (pSrcData : LPCVOID , SrcDataSize : SIZE_T , Flags : UINT , szComments : LPCSTR , ppDisassembly : * mut * mut ID3DBlob ,) -> HRESULT ; pub fn D3DDisassembleRegion (pSrcData : LPCVOID , SrcDataSize : SIZE_T , Flags : UINT , szComments : LPCSTR , StartByteOffset : SIZE_T , NumInsts : SIZE_T , pFinishByteOffset : * mut SIZE_T , ppDisassembly : * mut * mut ID3DBlob ,) -> HRESULT ; pub fn D3DCreateLinker (ppLinker : * mut * mut ID3D11Linker ,) -> HRESULT ; pub fn D3DLoadModule (pSrcData : LPCVOID , cbSrcDataSize : SIZE_T , ppModule : * mut * mut ID3D11Module ,) -> HRESULT ; pub fn D3DCreateFunctionLinkingGraph (uFlags : UINT , ppFunctionLinkingGraph : * mut * mut ID3D11FunctionLinkingGraph ,) -> HRESULT ; }
};
}
