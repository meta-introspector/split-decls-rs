// Generated macro for other_23425 (other)
macro_rules! Depcrate_um_commctrlother_23425 {
() => {
// Module: crate::um::commctrl
// Provides: {"other_23425"}
// Dependencies: {}
extern "system" { pub fn TaskDialogIndirect (pTaskConfig : * const TASKDIALOGCONFIG , pnButton : * mut c_int , pnRadioButton : * mut c_int , pfVerificationFlagChecked : * mut BOOL ,) -> HRESULT ; pub fn TaskDialog (hwndOwner : HWND , hInstance : HINSTANCE , pszWindowTitle : PCWSTR , pszMainInstruction : PCWSTR , pszContent : PCWSTR , dwCommonButtons : TASKDIALOG_COMMON_BUTTON_FLAGS , pszIcon : PCWSTR , pnButton : * mut c_int ,) -> HRESULT ; pub fn InitMUILanguage (uiLang : LANGID ,) ; pub fn GetMUILanguage () -> LANGID ; pub fn _TrackMouseEvent (lpEventTrack : LPTRACKMOUSEEVENT ,) -> BOOL ; }
};
}
