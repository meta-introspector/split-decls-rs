// Generated macro for macro_36329 (macro)
macro_rules! Depcrate_um_sapimacro_36329 {
() => {
// Module: crate::um::sapi
// Provides: {"macro_36329"}
// Dependencies: {}
RIDL ! { # [uuid (0xdf1b943c , 0x5838 , 0x4aa2 , 0x87 , 0x06 , 0xd7 , 0xcd , 0x5b , 0x33 , 0x34 , 0x99)] interface ISpRecognizer3 (ISpRecognizer3Vtbl) : IUnknown (IUnknownVtbl) { fn GetCategory (categoryType : SPCATEGORYTYPE , ppCategory : * mut * mut ISpRecoCategory ,) -> HRESULT , fn SetActiveCategory (pCategory : * mut ISpRecoCategory ,) -> HRESULT , fn GetActiveCategory (ppCategory : * mut * mut ISpRecoCategory ,) -> HRESULT , } }
};
}
