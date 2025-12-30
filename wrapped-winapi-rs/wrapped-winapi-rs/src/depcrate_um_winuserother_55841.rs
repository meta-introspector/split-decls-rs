// Generated macro for other_55841 (other)
macro_rules! Depcrate_um_winuserother_55841 {
() => {
// Module: crate::um::winuser
// Provides: {"other_55841"}
// Dependencies: {}
extern "system" { pub fn GetWindowFeedbackSetting (hwnd : HWND , feedback : FEEDBACK_TYPE , dwFlags : DWORD , pSize : * mut UINT32 , config : * mut VOID ,) -> BOOL ; pub fn SetWindowFeedbackSetting (hwnd : HWND , feedback : FEEDBACK_TYPE , dwFlags : DWORD , size : UINT32 , configuration : * const VOID ,) -> BOOL ; }
};
}
