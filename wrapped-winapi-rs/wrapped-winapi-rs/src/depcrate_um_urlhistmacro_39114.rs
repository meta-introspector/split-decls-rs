// Generated macro for macro_39114 (macro)
macro_rules! Depcrate_um_urlhistmacro_39114 {
() => {
// Module: crate::um::urlhist
// Provides: {"macro_39114"}
// Dependencies: {}
RIDL ! { # [uuid (0x3c374a41 , 0xbae4 , 0x11cf , 0xbf , 0x7d , 0x00 , 0xaa , 0x00 , 0x69 , 0x46 , 0xee)] interface IUrlHistoryStg (IUrlHistoryStgVtbl) : IUnknown (IUnknownVtbl) { fn AddUrl (pocsUrl : LPCOLESTR ,) -> HRESULT , fn DeleteUrl (pocsUrl : LPCOLESTR , dwFlags : DWORD ,) -> HRESULT , fn QueryUrl (pocsUrl : LPCOLESTR , dwFlags : DWORD , lpSTATURL : LPSTATURL ,) -> HRESULT , fn BindToObject (pocsUrl : LPCOLESTR , riid : REFIID , ppvOut : * mut * mut c_void ,) -> HRESULT , fn EnumUrls (ppEnum : * mut * mut IEnumSTATURL ,) -> HRESULT , } }
};
}
