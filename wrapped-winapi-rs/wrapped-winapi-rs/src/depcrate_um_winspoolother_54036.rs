// Generated macro for other_54036 (other)
macro_rules! Depcrate_um_winspoolother_54036 {
() => {
// Module: crate::um::winspool
// Provides: {"other_54036"}
// Dependencies: {}
extern "system" { pub fn GetCorePrinterDriversA (pszServer : LPCSTR , pszEnvironment : LPCSTR , pszzCoreDriverDependencies : LPCSTR , cCorePrinterDrivers : DWORD , pCorePrinterDrivers : PCORE_PRINTER_DRIVERA ,) -> HRESULT ; pub fn GetCorePrinterDriversW (pszServer : LPCWSTR , pszEnvironment : LPCWSTR , pszzCoreDriverDependencies : LPCWSTR , cCorePrinterDrivers : DWORD , pCorePrinterDrivers : PCORE_PRINTER_DRIVERW ,) -> HRESULT ; pub fn CorePrinterDriverInstalledA (pszServer : LPCSTR , pszEnvironment : LPCSTR , CoreDriverGUID : GUID , ftDriverDate : FILETIME , dwlDriverVersion : DWORDLONG , pbDriverInstalled : * mut BOOL ,) -> HRESULT ; pub fn CorePrinterDriverInstalledW (pszServer : LPCWSTR , pszEnvironment : LPCWSTR , CoreDriverGUID : GUID , ftDriverDate : FILETIME , dwlDriverVersion : DWORDLONG , pbDriverInstalled : * mut BOOL ,) -> HRESULT ; pub fn GetPrinterDriverPackagePathA (pszServer : LPCSTR , pszEnvironment : LPCSTR , pszLanguage : LPCSTR , pszPackageID : LPCSTR , pszDriverPackageCab : LPSTR , cchDriverPackageCab : DWORD , pcchRequiredSize : LPDWORD ,) -> HRESULT ; pub fn GetPrinterDriverPackagePathW (pszServer : LPCWSTR , pszEnvironment : LPCWSTR , pszLanguage : LPCWSTR , pszPackageID : LPCWSTR , pszDriverPackageCab : LPWSTR , cchDriverPackageCab : DWORD , pcchRequiredSize : LPDWORD ,) -> HRESULT ; pub fn DeletePrinterDriverPackageA (pszServer : LPCSTR , pszInfPath : LPCSTR , pszEnvironment : LPCSTR ,) -> HRESULT ; pub fn DeletePrinterDriverPackageW (pszServer : LPCWSTR , pszInfPath : LPCWSTR , pszEnvironment : LPCWSTR ,) -> HRESULT ; }
};
}
