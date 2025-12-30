// Generated macro for macro_40647 (macro)
macro_rules! Depcrate_um_wincodecsdkmacro_40647 {
() => {
// Module: crate::um::wincodecsdk
// Provides: {"macro_40647"}
// Dependencies: {}
RIDL ! { # [uuid (0xfeaa2a8d , 0xb3f3 , 0x43e4 , 0xb2 , 0x5c , 0xd1 , 0xde , 0x99 , 0x0a , 0x1a , 0xe1)] interface IWICMetadataBlockReader (IWICMetadataBlockReaderVtbl) : IUnknown (IUnknownVtbl) { fn GetContainerFormat (pguidContainerFormat : * mut GUID ,) -> HRESULT , fn GetCount (pcCount : * mut UINT ,) -> HRESULT , fn GetReaderByIndex (ppIMetadataReader : * mut * mut IWICMetadataReader ,) -> HRESULT , fn GetEnumerator (ppIEnumMetadata : * mut IEnumUnknown ,) -> HRESULT , } }
};
}
