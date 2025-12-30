// Generated macro for macro_37448 (macro)
macro_rules! Depcrate_um_setupapimacro_37448 {
() => {
// Module: crate::um::setupapi
// Provides: {"macro_37448"}
// Dependencies: {}
STRUCT ! { # [cfg_attr (target_arch = "x86" , repr (packed))] struct SP_INSTALLWIZARD_DATA { ClassInstallHeader : SP_CLASSINSTALL_HEADER , Flags : DWORD , DynamicPages : [HPROPSHEETPAGE ; MAX_INSTALLWIZARD_DYNAPAGES] , NumDynamicPages : DWORD , DynamicPageFlags : DWORD , PrivateFlags : DWORD , PrivateData : LPARAM , hwndWizardDlg : HWND , } }
};
}
