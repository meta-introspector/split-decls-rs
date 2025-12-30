// Generated macro for other_37775 (other)
macro_rules! Depcrate_um_setupapiother_37775 {
() => {
// Module: crate::um::setupapi
// Provides: {"other_37775"}
// Dependencies: {}
extern "system" { pub fn SetupInitDefaultQueueCallback (OwnerWindow : HWND ,) -> PVOID ; pub fn SetupInitDefaultQueueCallbackEx (OwnerWindow : HWND , AlternateProgressWindow : HWND , ProgressMessage : UINT , Reserved1 : DWORD , Reserved2 : PVOID ,) -> PVOID ; pub fn SetupTermDefaultQueueCallback (Context : PVOID ,) -> () ; pub fn SetupDefaultQueueCallbackA (Context : PVOID , Notification : UINT , Param1 : UINT_PTR , Param2 : UINT_PTR ,) -> UINT ; pub fn SetupDefaultQueueCallbackW (Context : PVOID , Notification : UINT , Param1 : UINT_PTR , Param2 : UINT_PTR ,) -> UINT ; }
};
}
