// Generated macro for macro_40657 (macro)
macro_rules! Depcrate_um_wincodecsdkmacro_40657 {
() => {
// Module: crate::um::wincodecsdk
// Provides: {"macro_40657"}
// Dependencies: {}
RIDL ! { # [uuid (0xb22e3fba , 0x3925 , 0x4323 , 0xb5 , 0xc1 , 0x9e , 0xbf , 0xc4 , 0x30 , 0xf2 , 0x36)] interface IWICMetadataWriterInfo (IWICMetadataWriterInfoVtbl) : IWICMetadataHandlerInfo (IWICMetadataHandlerInfoVtbl) { fn GetHeader (guidContainerFormat : REFGUID , cbSize : UINT , pHeader : * mut WICMetadataHeader , pcbActual : * mut UINT ,) -> HRESULT , fn CreateInstance (ppIWriter : * mut * mut IWICMetadataWriter ,) -> HRESULT , } }
};
}
