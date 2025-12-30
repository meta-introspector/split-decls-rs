// Generated macro for macro_33850 (macro)
macro_rules! Depcrate_um_objidlmacro_33850 {
() => {
// Module: crate::um::objidl
// Provides: {"macro_33850"}
// Dependencies: {}
RIDL ! { # [uuid (0x0000010e , 0x0000 , 0x0000 , 0xc0 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x46)] interface IDataObject (IDataObjectVtbl) : IUnknown (IUnknownVtbl) { fn GetData (pformatetcIn : * const FORMATETC , pmedium : * mut STGMEDIUM ,) -> HRESULT , fn GetDataHere (pformatetc : * const FORMATETC , pmedium : * mut STGMEDIUM ,) -> HRESULT , fn QueryGetData (pformatetc : * const FORMATETC ,) -> HRESULT , fn GetCanonicalFormatEtc (pformatetcIn : * const FORMATETC , pformatetcOut : * mut FORMATETC ,) -> HRESULT , fn SetData (pformatetc : * const FORMATETC , pformatetcOut : * const FORMATETC , fRelease : BOOL ,) -> HRESULT , fn EnumFormatEtc (dwDirection : DWORD , ppenumFormatEtc : * mut * mut IEnumFORMATETC ,) -> HRESULT , fn DAdvise (pformatetc : * const FORMATETC , advf : DWORD , pAdvSInk : * const IAdviseSink , pdwConnection : * mut DWORD ,) -> HRESULT , fn DUnadvise (dwConnection : DWORD ,) -> HRESULT , fn EnumDAdvise (ppenumAdvise : * const * const IEnumSTATDATA ,) -> HRESULT , } }
};
}
