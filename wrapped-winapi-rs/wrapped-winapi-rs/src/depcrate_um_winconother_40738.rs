// Generated macro for other_40738 (other)
macro_rules! Depcrate_um_winconother_40738 {
() => {
// Module: crate::um::wincon
// Provides: {"other_40738"}
// Dependencies: {}
extern "system" { pub fn SetConsoleDisplayMode (hConsoleOutput : HANDLE , dwFlags : DWORD , lpNewScreenBufferDimensions : PCOORD ,) -> BOOL ; pub fn GetConsoleWindow () -> HWND ; pub fn GetConsoleProcessList (lpdwProcessList : LPDWORD , dwProcessCount : DWORD ,) -> DWORD ; pub fn AddConsoleAliasA (Source : LPSTR , Target : LPSTR , ExeName : LPSTR ,) -> BOOL ; pub fn AddConsoleAliasW (Source : LPWSTR , Target : LPWSTR , ExeName : LPWSTR ,) -> BOOL ; pub fn GetConsoleAliasA (Source : LPSTR , TargetBuffer : LPSTR , TargetBufferLength : DWORD , ExeName : LPSTR ,) -> DWORD ; pub fn GetConsoleAliasW (Source : LPWSTR , TargetBuffer : LPWSTR , TargetBufferLength : DWORD , ExeName : LPWSTR ,) -> DWORD ; pub fn GetConsoleAliasesLengthA (ExeName : LPSTR ,) -> DWORD ; pub fn GetConsoleAliasesLengthW (ExeName : LPWSTR ,) -> DWORD ; pub fn GetConsoleAliasExesLengthA () -> DWORD ; pub fn GetConsoleAliasExesLengthW () -> DWORD ; pub fn GetConsoleAliasesA (AliasBuffer : LPSTR , AliasBufferLength : DWORD , ExeName : LPSTR ,) -> DWORD ; pub fn GetConsoleAliasesW (AliasBuffer : LPWSTR , AliasBufferLength : DWORD , ExeName : LPWSTR ,) -> DWORD ; pub fn GetConsoleAliasExesA (ExeNameBuffer : LPSTR , ExeNameBufferLength : DWORD ,) -> DWORD ; pub fn GetConsoleAliasExesW (ExeNameBuffer : LPWSTR , ExeNameBufferLength : DWORD ,) -> DWORD ; }
};
}
