// Generated macro for macro_27960 (macro)
macro_rules! Depcrate_um_dwritemacro_27960 {
() => {
// Module: crate::um::dwrite
// Provides: {"macro_27960"}
// Dependencies: {}
RIDL ! { # [uuid (0xeaf3a2da , 0xecf4 , 0x4d24 , 0xb6 , 0x44 , 0xb3 , 0x4f , 0x68 , 0x42 , 0x02 , 0x4b)] interface IDWritePixelSnapping (IDWritePixelSnappingVtbl) : IUnknown (IUnknownVtbl) { fn IsPixelSnappingDisabled (clientDrawingContext : * mut c_void , isDisabled : * mut BOOL ,) -> HRESULT , fn GetCurrentTransform (clientDrawingContext : * mut c_void , transform : * mut DWRITE_MATRIX ,) -> HRESULT , fn GetPixelsPerDip (clientDrawingContext : * mut c_void , pixelsPerDip : * mut FLOAT ,) -> HRESULT , } }
};
}
