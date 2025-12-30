// Generated macro for macro_23738 (macro)
macro_rules! Depcrate_um_commoncontrolsmacro_23738 {
() => {
// Module: crate::um::commoncontrols
// Provides: {"macro_23738"}
// Dependencies: {}
RIDL ! { # [uuid (0x192b9d83 , 0x58fc , 0x457b , 0x90 , 0xa0 , 0x2b , 0x82 , 0xa8 , 0xb5 , 0xda , 0xe1)] interface IImageList2 (IImageList2Vtbl) : IImageList (IImageListVtbl) { fn Resize (cxNewIconSize : c_int , cyNewIconSize : c_int ,) -> HRESULT , fn GetOriginalSize (iImage : c_int , dwFlags : DWORD , pcx : * mut c_int , pcy : * mut c_int ,) -> HRESULT , fn SetOriginalSize (iImage : c_int , cx : c_int , cy : c_int ,) -> HRESULT , fn SetCallback (punk : * mut IUnknown ,) -> HRESULT , fn GetCallback (riid : REFIID , ppv : * mut * mut c_void ,) -> HRESULT , fn ForceImagePresent (iImage : c_int , dwFlags : DWORD ,) -> HRESULT , fn DiscardImages (iFirstImage : c_int , iLastImage : c_int , dwFlags : DWORD ,) -> HRESULT , fn PreloadImages (pimldp : * mut IMAGELISTDRAWPARAMS ,) -> HRESULT , fn GetStatistics (pils : * mut IMAGELISTSTATS ,) -> HRESULT , fn Initialize (cx : c_int , cy : c_int , flags : UINT , cInitial : c_int , cGrow : c_int ,) -> HRESULT , fn Replace2 (i : c_int , hbmImage : HBITMAP , hbmMask : HBITMAP , punk : * mut IUnknown , dwFlags : DWORD ,) -> HRESULT , fn ReplaceFromImageList (i : c_int , pil : * mut IImageList , iSrc : c_int , punk : * mut IUnknown , dwFlags : DWORD ,) -> HRESULT , } }
};
}
