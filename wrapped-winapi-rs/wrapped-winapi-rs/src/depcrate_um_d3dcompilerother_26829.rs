// Generated macro for other_26829 (other)
macro_rules! Depcrate_um_d3dcompilerother_26829 {
() => {
// Module: crate::um::d3dcompiler
// Provides: {"other_26829"}
// Dependencies: {}
extern "system" { pub fn D3DCompile (pSrcData : LPCVOID , SrcDataSize : SIZE_T , pSourceName : LPCSTR , pDefines : * const D3D_SHADER_MACRO , pInclude : * mut ID3DInclude , pEntrypoint : LPCSTR , pTarget : LPCSTR , Flags1 : UINT , Flags2 : UINT , ppCode : * mut * mut ID3DBlob , ppErrorMsgs : * mut * mut ID3DBlob ,) -> HRESULT ; }
};
}
