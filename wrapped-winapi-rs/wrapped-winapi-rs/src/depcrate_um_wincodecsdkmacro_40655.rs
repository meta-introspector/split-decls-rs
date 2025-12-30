// Generated macro for macro_40655 (macro)
macro_rules! Depcrate_um_wincodecsdkmacro_40655 {
() => {
// Module: crate::um::wincodecsdk
// Provides: {"macro_40655"}
// Dependencies: {}
RIDL ! { # [uuid (0xeebf1f5b , 0x07c1 , 0x4447 , 0xa3 , 0xab , 0x22 , 0xac , 0xaf , 0x78 , 0xa8 , 0x04)] interface IWICMetadataReaderInfo (IWICMetadataReaderInfoVtbl) : IWICMetadataHandlerInfo (IWICMetadataHandlerInfoVtbl) { fn GetPatterns (guidContainerFormat : REFGUID , cbSize : UINT , pPattern : * mut WICMetadataPattern , pcCount : * mut UINT , pcbActual : * mut UINT ,) -> HRESULT , fn MatchesPattern (guidContainerFormat : REFGUID , pIStream : * mut IStream , pfMatches : * mut BOOL ,) -> HRESULT , fn CreateInstance (ppIReader : * mut * mut IWICMetadataReader ,) -> HRESULT , } }
};
}
