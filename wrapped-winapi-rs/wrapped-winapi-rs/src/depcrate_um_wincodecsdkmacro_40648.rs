// Generated macro for macro_40648 (macro)
macro_rules! Depcrate_um_wincodecsdkmacro_40648 {
() => {
// Module: crate::um::wincodecsdk
// Provides: {"macro_40648"}
// Dependencies: {}
RIDL ! { # [uuid (0x08fb9676 , 0xb444 , 0x41e8 , 0x8d , 0xbe , 0x6a , 0x53 , 0xa5 , 0x42 , 0xbf , 0xf1)] interface IWICMetadataBlockWriter (IWICMetadataBlockWriterVtbl) : IWICMetadataBlockReader (IWICMetadataBlockReaderVtbl) { fn InitializeFromBlockReader (pIMDBlockReader : * mut IWICMetadataBlockReader ,) -> HRESULT , fn GetWriterByIndex (ppIMetadataWriter : * mut * mut IWICMetadataWriter ,) -> HRESULT , fn AddWriter (pIMetadataWriter : * mut IWICMetadataWriter ,) -> HRESULT , fn SetWriterByIndex (pIMetadataWriter : * mut IWICMetadataWriter ,) -> HRESULT , fn RemoveWriterByIndex (nIndex : UINT ,) -> HRESULT , } }
};
}
