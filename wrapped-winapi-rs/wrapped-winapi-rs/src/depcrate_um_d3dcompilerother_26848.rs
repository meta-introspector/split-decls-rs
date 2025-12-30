// Generated macro for other_26848 (other)
macro_rules! Depcrate_um_d3dcompilerother_26848 {
() => {
// Module: crate::um::d3dcompiler
// Provides: {"other_26848"}
// Dependencies: {}
extern "system" { pub fn D3DGetBlobPart (pSrcData : LPCVOID , SrcDataSize : SIZE_T , Part : D3D_BLOB_PART , Flags : UINT , ppPart : * mut * mut ID3DBlob ,) -> HRESULT ; pub fn D3DSetBlobPart (pSrcData : LPCVOID , SrcDataSize : SIZE_T , Part : D3D_BLOB_PART , Flags : UINT , pPart : LPCVOID , PartSize : SIZE_T , ppNewShader : * mut * mut ID3DBlob ,) -> HRESULT ; pub fn D3DCreateBlob (Size : SIZE_T , ppBlob : * mut * mut ID3DBlob ,) -> HRESULT ; }
};
}
