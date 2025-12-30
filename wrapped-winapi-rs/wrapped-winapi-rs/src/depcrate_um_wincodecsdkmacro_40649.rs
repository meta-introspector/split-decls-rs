// Generated macro for macro_40649 (macro)
macro_rules! Depcrate_um_wincodecsdkmacro_40649 {
() => {
// Module: crate::um::wincodecsdk
// Provides: {"macro_40649"}
// Dependencies: {}
RIDL ! { # [uuid (0x9204fe99 , 0xd8fc , 0x4fd5 , 0xa0 , 0x01 , 0x95 , 0x36 , 0xb0 , 0x67 , 0xa8 , 0x99)] interface IWICMetadataReader (IWICMetadataReaderVtbl) : IUnknown (IUnknownVtbl) { fn GetMetadataFormat (pguidMetadataFormat : * mut GUID ,) -> HRESULT , fn GetMetadataHandlerInfo (ppIHandler : * mut * mut IWICMetadataHandlerInfo ,) -> HRESULT , fn GetCount (pcCount : * mut UINT ,) -> HRESULT , fn GetValueByIndex (nIndex : UINT , pvarSchema : * mut PROPVARIANT , pvarId : * mut PROPVARIANT , pvarValue : * mut PROPVARIANT ,) -> HRESULT , fn GetValue (pvarSchema : * const PROPVARIANT , pvarId : * const PROPVARIANT , pvarValue : * mut PROPVARIANT ,) -> HRESULT , fn GetEnumerator (ppIEnumMetadata : * mut * mut IWICEnumMetadataItem ,) -> HRESULT , } }
};
}
