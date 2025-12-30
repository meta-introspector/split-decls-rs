// Generated macro for macro_36758 (macro)
macro_rules! Depcrate_um_sapi53macro_36758 {
() => {
// Module: crate::um::sapi53
// Provides: {"macro_36758"}
// Dependencies: {}
RIDL ! { # [uuid (0x8fc6d974 , 0xc81e , 0x4098 , 0x93 , 0xc5 , 0x01 , 0x47 , 0xf6 , 0x1e , 0xd4 , 0xd3)] interface ISpRecognizer2 (ISpRecognizer2Vtbl) : IUnknown (IUnknownVtbl) { fn EmulateRecognitionEx (pPhrase : * mut ISpPhrase , dwCompareFlags : DWORD ,) -> HRESULT , fn SetTrainingState (fDoingTraining : BOOL , fAdaptFromTrainingData : BOOL ,) -> HRESULT , fn ResetAcousticModelAdaptation () -> HRESULT , } }
};
}
