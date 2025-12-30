// Generated macro for macro_40446 (macro)
macro_rules! Depcrate_um_wincodecmacro_40446 {
() => {
// Module: crate::um::wincodec
// Provides: {"macro_40446"}
// Dependencies: {}
RIDL ! { # [uuid (0x30989668 , 0xe1c9 , 0x4597 , 0xb3 , 0x95 , 0x45 , 0x8e , 0xed , 0xb8 , 0x08 , 0xdf)] interface IWICMetadataQueryReader (IWICMetadataQueryReaderVtbl) : IUnknown (IUnknownVtbl) { fn GetContainerFormat (pguidContainerFormat : * mut GUID ,) -> HRESULT , fn GetLocation (cchMaxLength : UINT , wzNamespace : * mut WCHAR , pcchActualLength : * mut UINT ,) -> HRESULT , fn GetMetadataByName (wzName : LPCWSTR , pvarValue : * mut PROPVARIANT ,) -> HRESULT , fn GetEnumerator (ppIEnumString : * mut * mut IEnumString ,) -> HRESULT , } }
};
}
