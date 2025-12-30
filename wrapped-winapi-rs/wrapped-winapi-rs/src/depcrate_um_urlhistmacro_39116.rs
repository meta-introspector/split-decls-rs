// Generated macro for macro_39116 (macro)
macro_rules! Depcrate_um_urlhistmacro_39116 {
() => {
// Module: crate::um::urlhist
// Provides: {"macro_39116"}
// Dependencies: {}
RIDL ! { # [uuid (0xafa0dc11 , 0xc313 , 0x11d0 , 0x83 , 0x1a , 0x00 , 0xc0 , 0x4f , 0xd5 , 0xae , 0x38)] interface IUrlHistoryStg2 (IUrlHistoryStg2Vtbl) : IUrlHistoryStg (IUrlHistoryStgVtbl) { fn AddUrlAndNotify (pocsUrl : LPCOLESTR , pocsTitle : LPCOLESTR , dwFlags : DWORD , fWriteHistory : BOOL , poctNotify : * mut IOleCommandTarget , punkISFolder : * mut IUnknown ,) -> HRESULT , fn ClearHistory () -> HRESULT , } }
};
}
