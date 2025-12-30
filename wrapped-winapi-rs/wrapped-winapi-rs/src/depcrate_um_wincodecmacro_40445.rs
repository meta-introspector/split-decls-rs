// Generated macro for macro_40445 (macro)
macro_rules! Depcrate_um_wincodecmacro_40445 {
() => {
// Module: crate::um::wincodec
// Provides: {"macro_40445"}
// Dependencies: {}
RIDL ! { # [uuid (0xdc2bb46d , 0x3f07 , 0x481e , 0x86 , 0x25 , 0x22 , 0x0c , 0x4a , 0xed , 0xbb , 0x33)] interface IWICEnumMetadataItem (IWICEnumMetadataItemVtbl) : IUnknown (IUnknownVtbl) { fn Next (celt : ULONG , rgeltSchema : * mut PROPVARIANT , rgeltId : * mut PROPVARIANT , rgeltValue : * mut PROPVARIANT , pceltFetched : * mut ULONG ,) -> HRESULT , fn Skip (celt : ULONG ,) -> HRESULT , fn Reset () -> HRESULT , fn Clone (ppIEnumMetadataItem : * mut * mut IWICEnumMetadataItem ,) -> HRESULT , } }
};
}
