// Generated macro for macro_37483 (macro)
macro_rules! Depcrate_um_setupapimacro_37483 {
() => {
// Module: crate::um::setupapi
// Provides: {"macro_37483"}
// Dependencies: {}
STRUCT ! { # [cfg_attr (target_arch = "x86" , repr (packed))] struct SP_NEWDEVICEWIZARD_DATA { ClassInstallHeader : SP_CLASSINSTALL_HEADER , Flags : DWORD , DynamicPages : [HPROPSHEETPAGE ; MAX_INSTALLWIZARD_DYNAPAGES] , NumDynamicPages : DWORD , hwndWizardDlg : HWND , } }
};
}
