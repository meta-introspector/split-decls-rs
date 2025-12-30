// Generated macro for macro_36470 (macro)
macro_rules! Depcrate_um_sapi51macro_36470 {
() => {
// Module: crate::um::sapi51
// Provides: {"macro_36470"}
// Dependencies: {}
RIDL ! { # [uuid (0xda41a7c2 , 0x5383 , 0x4db2 , 0x91 , 0x6b , 0x6c , 0x17 , 0x19 , 0xe3 , 0xdb , 0x58)] interface ISpLexicon (ISpLexiconVtbl) : IUnknown (IUnknownVtbl) { fn GetPronunciations (pszWord : LPCWSTR , LangID : WORD , dwFlags : DWORD , pWordPronunciationList : * mut SPWORDPRONUNCIATIONLIST ,) -> HRESULT , fn AddPronunciation (pszWord : LPCWSTR , LangID : WORD , ePartOfSpeech : SPPARTOFSPEECH , pszPronunciation : PCSPPHONEID ,) -> HRESULT , fn RemovePronunciation (pszWord : LPCWSTR , LangID : WORD , ePartOfSpeech : SPPARTOFSPEECH , pszPronunciation : PCSPPHONEID ,) -> HRESULT , fn GetGeneration (pdwGeneration : * mut DWORD ,) -> HRESULT , fn GetGenerationChange (dwFlags : DWORD , pdwGeneration : * mut DWORD , pWordList : * mut SPWORDLIST ,) -> HRESULT , fn GetWords (dwFlags : DWORD , pdwGeneration : * mut DWORD , pdwCookie : * mut DWORD , pWordList : * mut SPWORDLIST ,) -> HRESULT , } }
};
}
