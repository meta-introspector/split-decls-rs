// Generated macro for other_31796 (other)
macro_rules! Depcrate_um_memoryapiother_31796 {
() => {
// Module: crate::um::memoryapi
// Provides: {"other_31796"}
// Dependencies: {}
extern "system" { pub fn GetSystemFileCacheSize (lpMinimumFileCacheSize : PSIZE_T , lpMaximumFileCacheSize : PSIZE_T , lpFlags : PDWORD ,) -> BOOL ; pub fn SetSystemFileCacheSize (MinimumFileCacheSize : SIZE_T , MaximumFileCacheSize : SIZE_T , Flags : DWORD ,) -> BOOL ; pub fn CreateFileMappingNumaW (hFile : HANDLE , lpFileMappingAttributes : LPSECURITY_ATTRIBUTES , flProtect : DWORD , dwMaximumSizeHigh : DWORD , dwMaximumSizeLow : DWORD , lpName : LPCWSTR , nndPreferred : DWORD ,) -> HANDLE ; }
};
}
