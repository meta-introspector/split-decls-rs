// Generated macro for macro_27201 (macro)
macro_rules! Depcrate_um_dcompmacro_27201 {
() => {
// Module: crate::um::dcomp
// Provides: {"macro_27201"}
// Dependencies: {}
RIDL ! { # [uuid (0xbb8a4953 , 0x2c99 , 0x4f5a , 0x96 , 0xf5 , 0x48 , 0x19 , 0x02 , 0x7f , 0xa3 , 0xac)] interface IDCompositionSurface (IDCompositionSurfaceVtbl) : IUnknown (IUnknownVtbl) { fn BeginDraw (updateRect : * const RECT , iid : REFIID , updateObject : * mut * mut c_void , updateOffset : * mut POINT ,) -> HRESULT , fn EndDraw () -> HRESULT , fn SuspendDraw () -> HRESULT , fn ResumeDraw () -> HRESULT , fn Scroll (scrollRect : * const RECT , clipRect : * const RECT , offsetX : c_int , offsetY : c_int ,) -> HRESULT , } }
};
}
