// Generated macro for callback (function)
macro_rules! Depcratecallback {
() => {
// Module: crate
// Provides: {"callback"}
// Dependencies: {}
extern "system" fn callback (_ : HWND , notification : TASKDIALOG_NOTIFICATIONS , _ : WPARAM , _ : LPARAM , _ : isize ,) -> HRESULT { if notification == TDN_BUTTON_CLICKED { println ! ("button clicked") ; } HRESULT (0) }
};
}
