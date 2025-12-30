// Generated macro for other_26833 (other)
macro_rules! Depcrate_um_d3dcompilerother_26833 {
() => {
// Module: crate::um::d3dcompiler
// Provides: {"other_26833"}
// Dependencies: {}
extern "system" { pub fn D3DCompile2 (pSrcData : LPCVOID , SrcDataSize : SIZE_T , pSourceName : LPCSTR , pDefines : * const D3D_SHADER_MACRO , pInclude : * mut ID3DInclude , pEntrypoint : LPCSTR , pTarget : LPCSTR , Flags1 : UINT , Flags2 : UINT , SecondaryDataFlags : UINT , pSecondaryData : LPCVOID , SecondaryDataSize : SIZE_T , ppCode : * mut * mut ID3DBlob , ppErrorMsgs : * mut * mut ID3DBlob ,) -> HRESULT ; pub fn D3DCompileFromFile (pFileName : LPCWSTR , pDefines : * const D3D_SHADER_MACRO , pInclude : * mut ID3DInclude , pEntrypoint : LPCSTR , pTarget : LPCSTR , Flags1 : UINT , Flags2 : UINT , ppCode : * mut * mut ID3DBlob , ppErrorMsgs : * mut * mut ID3DBlob ,) -> HRESULT ; pub fn D3DPreprocess (pSrcData : LPCVOID , SrcDataSize : SIZE_T , pSourceName : LPCSTR , pDefines : * const D3D_SHADER_MACRO , pInclude : * mut ID3DInclude , ppCodeText : * mut * mut ID3DBlob , ppErrorMsgs : * mut * mut ID3DBlob ,) -> HRESULT ; pub fn D3DGetDebugInfo (pSrcData : LPCVOID , SrcDataSize : SIZE_T , ppDebugInfo : * mut * mut ID3DBlob ,) -> HRESULT ; pub fn D3DReflect (pSrcData : LPCVOID , SrcDataSize : SIZE_T , pInterface : REFIID , ppReflector : * mut * mut c_void ,) -> HRESULT ; pub fn D3DReflectLibrary (pSrcData : LPCVOID , SrcDataSize : SIZE_T , riid : REFIID , ppReflector : * mut LPVOID ,) -> HRESULT ; }
};
}
