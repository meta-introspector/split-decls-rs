// Generated macro for other_40088 (other)
macro_rules! Depcrate_um_winbaseother_40088 {
() => {
// Module: crate::um::winbase
// Provides: {"other_40088"}
// Dependencies: {}
extern "system" { pub fn GlobalAlloc (uFlags : UINT , dwBytes : SIZE_T ,) -> HGLOBAL ; pub fn GlobalReAlloc (hMem : HGLOBAL , dwBytes : SIZE_T , uFlags : UINT ,) -> HGLOBAL ; pub fn GlobalSize (hMem : HGLOBAL ,) -> SIZE_T ; pub fn GlobalFlags (hMem : HGLOBAL ,) -> UINT ; pub fn GlobalLock (hMem : HGLOBAL ,) -> LPVOID ; pub fn GlobalHandle (pMem : LPCVOID ,) -> HGLOBAL ; pub fn GlobalUnlock (hMem : HGLOBAL ,) -> BOOL ; pub fn GlobalFree (hMem : HGLOBAL ,) -> HGLOBAL ; pub fn GlobalCompact (dwMinFree : DWORD ,) -> SIZE_T ; pub fn GlobalFix (hMem : HGLOBAL ,) ; pub fn GlobalUnfix (hMem : HGLOBAL ,) ; pub fn GlobalWire (hMem : HGLOBAL ,) -> LPVOID ; pub fn GlobalUnWire (hMem : HGLOBAL ,) -> BOOL ; pub fn GlobalMemoryStatus (lpBuffer : LPMEMORYSTATUS ,) ; pub fn LocalAlloc (uFlags : UINT , uBytes : SIZE_T ,) -> HLOCAL ; pub fn LocalReAlloc (hMem : HLOCAL , uBytes : SIZE_T , uFlags : UINT ,) -> HLOCAL ; pub fn LocalLock (hMem : HLOCAL ,) -> LPVOID ; pub fn LocalHandle (pMem : LPCVOID ,) -> HLOCAL ; pub fn LocalUnlock (hMem : HLOCAL ,) -> BOOL ; pub fn LocalSize (hMem : HLOCAL ,) -> SIZE_T ; pub fn LocalFlags (hMem : HLOCAL ,) -> UINT ; pub fn LocalFree (hMem : HLOCAL ,) -> HLOCAL ; pub fn LocalShrink (hMem : HLOCAL , cbNewSize : UINT ,) -> SIZE_T ; pub fn LocalCompact (uMinFree : UINT ,) -> SIZE_T ; }
};
}
