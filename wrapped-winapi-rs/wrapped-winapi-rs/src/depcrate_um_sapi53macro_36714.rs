// Generated macro for macro_36714 (macro)
macro_rules! Depcrate_um_sapi53macro_36714 {
() => {
// Module: crate::um::sapi53
// Provides: {"macro_36714"}
// Dependencies: {}
RIDL ! { # [uuid (0x3df681e2 , 0xea56 , 0x11d9 , 0x8b , 0xde , 0xf6 , 0x6b , 0xad , 0x1e , 0x3f , 0x3a)] interface ISpShortcut (ISpShortcutVtbl) : IUnknown (IUnknownVtbl) { fn AddShortcut (pszDisplay : LPCWSTR , LangID : WORD , pszSpoken : LPCWSTR , shType : SPSHORTCUTTYPE ,) -> HRESULT , fn RemoveShortcut (pszDisplay : LPCWSTR , LangID : WORD , pszSpoken : LPCWSTR , shType : SPSHORTCUTTYPE ,) -> HRESULT , fn GetShortcuts (LangId : WORD , pShortcutpairList : * mut SPSHORTCUTPAIRLIST ,) -> HRESULT , fn GetGeneration (pdwGeneration : * mut DWORD ,) -> HRESULT , fn GetWordsFromGenerationChange (pdwGeneration : * mut DWORD , pWordList : * mut SPWORDLIST ,) -> HRESULT , fn GetWords (pdwGeneration : * mut DWORD , pdwCookie : * mut DWORD , pWordList : * mut SPWORDLIST ,) -> HRESULT , fn GetShortcutsForGeneration (pdwGeneration : * mut DWORD , pdwCookie : * mut DWORD , pShortcutpairList : * mut SPSHORTCUTPAIRLIST ,) -> HRESULT , fn GetGenerationChange (pdwGeneration : * mut DWORD , pShortcutpairList : * mut SPSHORTCUTPAIRLIST ,) -> HRESULT , } }
};
}
