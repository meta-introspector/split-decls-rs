// Generated macro for other_38238 (other)
macro_rules! Depcrate_um_shellapiother_38238 {
() => {
// Module: crate::um::shellapi
// Provides: {"other_38238"}
// Dependencies: {}
extern "system" { pub fn SHSetLocalizedName (pszPath : PCWSTR , pszResModule : PCWSTR , idsRes : c_int ,) -> HRESULT ; pub fn SHRemoveLocalizedName (pszPath : PCWSTR ,) -> HRESULT ; pub fn SHGetLocalizedName (pszPath : PCWSTR , pszResModule : PWSTR , cch : UINT , pidsRes : * mut c_int ,) -> HRESULT ; }
};
}
