// Generated macro for macro_40658 (macro)
macro_rules! Depcrate_um_wincodecsdkmacro_40658 {
() => {
// Module: crate::um::wincodecsdk
// Provides: {"macro_40658"}
// Dependencies: {}
RIDL ! { # [uuid (0x412d0c3a , 0x9650 , 0x44fa , 0xaf , 0x5b , 0xdd , 0x2a , 0x06 , 0xc8 , 0xe8 , 0xfb)] interface IWICComponentFactory (IWICComponentFactoryVtbl) : IWICImagingFactory (IWICImagingFactoryVtbl) { fn CreateMetadataReader (guidMetadataFormat : REFGUID , pguidVendor : * const GUID , dwOptions : DWORD , pIStream : * mut IStream , ppIReader : * mut * mut IWICMetadataReader ,) -> HRESULT , fn CreateMetadataReaderFromContainer (guidContainerFormat : REFGUID , pguidVendor : * const GUID , dwOptions : DWORD , pIStream : * mut IStream , ppIReader : * mut * mut IWICMetadataReader ,) -> HRESULT , fn CreateMetadataWriter (guidMetadataFormat : REFGUID , pguidVendor : * const GUID , dwMetadataOptions : DWORD , ppIWriter : * mut * mut IWICMetadataWriter ,) -> HRESULT , fn CreateMetadataWriterFromReader (pIReader : * mut IWICMetadataReader , pguidVendor : * const GUID , ppIWriter : * mut * mut IWICMetadataWriter ,) -> HRESULT , fn CreateQueryReaderFromBlockReader (pIBlockReader : * mut IWICMetadataBlockReader , ppIQueryReader : * mut * mut IWICMetadataQueryReader ,) -> HRESULT , fn CreateQueryWriterFromBlockWriter (pIBlockWriter : * mut IWICMetadataBlockWriter , ppIQueryWriter : * mut * mut IWICMetadataQueryWriter ,) -> HRESULT , fn CreateEncoderPropertyBag (ppropOptions : * mut PROPBAG2 , cCount : UINT , ppIPropertyBag : * mut * mut IPropertyBag2 ,) -> HRESULT , } }
};
}
