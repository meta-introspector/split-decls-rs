// Generated macro for macro_36821 (macro)
macro_rules! Depcrate_um_sapiddkmacro_36821 {
() => {
// Module: crate::um::sapiddk
// Provides: {"macro_36821"}
// Dependencies: {}
RIDL ! { # [uuid (0xdefd682a , 0xfe0a , 0x42b9 , 0xbf , 0xa1 , 0x56 , 0xd3 , 0xd6 , 0xce , 0xcf , 0xaf)] interface ISpPrivateEngineCallEx (ISpPrivateEngineCallExVtbl) : IUnknown (IUnknownVtbl) { fn CallEngineSynchronize (pInFrame : * const c_void , ulInFrameSize : ULONG , ppCoMemOutFrame : * mut * mut c_void , pulOutFrameSize : * mut ULONG ,) -> HRESULT , fn CallEngineImmediate (pInFrame : * const c_void , ulInFrameSize : ULONG , ppCoMemOutFrame : * mut * mut c_void , pulOutFrameSize : * mut ULONG ,) -> HRESULT , } }
};
}
