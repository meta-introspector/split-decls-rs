// Generated macro for other_53905 (other)
macro_rules! Depcrate_um_winspoolother_53905 {
() => {
// Module: crate::um::winspool
// Provides: {"other_53905"}
// Dependencies: {}
extern "system" { pub fn AddPrintProvidorA (pName : LPSTR , Level : DWORD , pProvidorInfo : LPBYTE ,) -> BOOL ; pub fn AddPrintProvidorW (pName : LPWSTR , Level : DWORD , pProvidorInfo : LPBYTE ,) -> BOOL ; pub fn DeletePrintProvidorA (pName : LPSTR , pEnvironment : LPSTR , pPrintProvidorName : LPSTR ,) -> BOOL ; pub fn DeletePrintProvidorW (pName : LPWSTR , pEnvironment : LPWSTR , pPrintProvidorName : LPWSTR ,) -> BOOL ; pub fn IsValidDevmodeA (pDevmode : PDEVMODEA , DevmodeSize : size_t ,) -> BOOL ; pub fn IsValidDevmodeW (pDevmode : PDEVMODEW , DevmodeSize : size_t ,) -> BOOL ; }
};
}
