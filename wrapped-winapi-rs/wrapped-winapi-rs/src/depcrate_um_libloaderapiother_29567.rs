// Generated macro for other_29567 (other)
macro_rules! Depcrate_um_libloaderapiother_29567 {
() => {
// Module: crate::um::libloaderapi
// Provides: {"other_29567"}
// Dependencies: {}
extern "system" { pub fn AddDllDirectory (NewDirectory : PCWSTR ,) -> DLL_DIRECTORY_COOKIE ; pub fn RemoveDllDirectory (Cookie : DLL_DIRECTORY_COOKIE ,) -> BOOL ; pub fn SetDefaultDllDirectories (DirectoryFlags : DWORD ,) -> BOOL ; pub fn EnumResourceLanguagesExA (hModule : HMODULE , lpType : LPCSTR , lpName : LPCSTR , lpEnumFunc : ENUMRESLANGPROCA , lParam : LONG_PTR , dwFlags : DWORD , LangId : LANGID ,) -> BOOL ; pub fn EnumResourceLanguagesExW (hModule : HMODULE , lpType : LPCWSTR , lpName : LPCWSTR , lpEnumFunc : ENUMRESLANGPROCW , lParam : LONG_PTR , dwFlags : DWORD , LangId : LANGID ,) -> BOOL ; pub fn EnumResourceNamesExA (hModule : HMODULE , lpType : LPCSTR , lpEnumFunc : ENUMRESNAMEPROCA , lParam : LONG_PTR , dwFlags : DWORD , LangId : LANGID ,) -> BOOL ; pub fn EnumResourceNamesExW (hModule : HMODULE , lpType : LPCWSTR , lpEnumFunc : ENUMRESNAMEPROCW , lParam : LONG_PTR , dwFlags : DWORD , LangId : LANGID ,) -> BOOL ; pub fn EnumResourceTypesExA (hModule : HMODULE , lpEnumFunc : ENUMRESTYPEPROCA , lParam : LONG_PTR , dwFlags : DWORD , LangId : LANGID ,) -> BOOL ; pub fn EnumResourceTypesExW (hModule : HMODULE , lpEnumFunc : ENUMRESTYPEPROCW , lParam : LONG_PTR , dwFlags : DWORD , LangId : LANGID ,) -> BOOL ; pub fn FindResourceW (hModule : HMODULE , lpName : LPCWSTR , lpType : LPCWSTR ,) -> HRSRC ; pub fn LoadLibraryA (lpFileName : LPCSTR ,) -> HMODULE ; pub fn LoadLibraryW (lpFileName : LPCWSTR ,) -> HMODULE ; pub fn EnumResourceNamesW (hModule : HMODULE , lpType : LPCWSTR , lpEnumFunc : ENUMRESNAMEPROCW , lParam : LONG_PTR ,) -> BOOL ; }
};
}
