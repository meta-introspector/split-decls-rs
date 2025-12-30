// Generated macro for macro_40441 (macro)
macro_rules! Depcrate_um_wincodecmacro_40441 {
() => {
// Module: crate::um::wincodec
// Provides: {"macro_40441"}
// Dependencies: {}
RIDL ! { # [uuid (0x3c613a02 , 0x34b2 , 0x44ea , 0x9a , 0x7c , 0x45 , 0xae , 0xa9 , 0xc6 , 0xfd , 0x6d)] interface IWICColorContext (IWICColorContextVtbl) : IUnknown (IUnknownVtbl) { fn InitializeFromFilename (wzFilename : LPCWSTR ,) -> HRESULT , fn InitializeFromMemory (pbBuffer : * const BYTE , cbBufferSize : UINT ,) -> HRESULT , fn InitializeFromExifColorSpace (value : UINT ,) -> HRESULT , fn GetType (pType : * mut WICColorContextType ,) -> HRESULT , fn GetProfileBytes (cbBuffer : UINT , pbBuffer : * mut BYTE , pcbActual : * mut UINT ,) -> HRESULT , fn GetExifColorSpace (pValue : * mut UINT ,) -> HRESULT , } }
};
}
