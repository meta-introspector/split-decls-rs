// Generated macro for macro_36885 (macro)
macro_rules! Depcrate_um_sapiddk51macro_36885 {
() => {
// Module: crate::um::sapiddk51
// Provides: {"macro_36885"}
// Dependencies: {}
RIDL ! { # [uuid (0x8e7c791e , 0x4467 , 0x11d3 , 0x97 , 0x23 , 0x00 , 0xc0 , 0x4f , 0x72 , 0xdb , 0x08)] interface _ISpPrivateEngineCall (_ISpPrivateEngineCallVtbl) : IUnknown (IUnknownVtbl) { fn CallEngine (pCallFrame : * mut c_void , ulCallFrameSize : ULONG ,) -> HRESULT , fn CallEngineEx (pInFrame : * const c_void , ulInFrameSize : ULONG , ppCoMemOutFrame : * mut * mut c_void , pulOutFrameSize : * mut ULONG ,) -> HRESULT , } }
};
}
