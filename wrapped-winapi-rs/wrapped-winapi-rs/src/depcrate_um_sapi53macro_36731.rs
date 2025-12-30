// Generated macro for macro_36731 (macro)
macro_rules! Depcrate_um_sapi53macro_36731 {
() => {
// Module: crate::um::sapi53
// Provides: {"macro_36731"}
// Dependencies: {}
RIDL ! { # [uuid (0x27cac6c4 , 0x88f2 , 0x41f2 , 0x88 , 0x17 , 0x0c , 0x95 , 0xe5 , 0x9f , 0x1e , 0x6e)] interface ISpRecoResult2 (ISpRecoResult2Vtbl) : ISpRecoResult (ISpRecoResultVtbl) { fn CommitAlternate (pPhraseAlt : * mut ISpPhraseAlt , ppNewResult : * mut * mut ISpRecoResult ,) -> HRESULT , fn CommitText (ulStartElement : ULONG , cElements : ULONG , pszCorrectedData : LPCWSTR , eCommitFlags : DWORD ,) -> HRESULT , fn SetTextFeedback (pszFeedback : LPCWSTR , fSuccessful : BOOL ,) -> HRESULT , } }
};
}
