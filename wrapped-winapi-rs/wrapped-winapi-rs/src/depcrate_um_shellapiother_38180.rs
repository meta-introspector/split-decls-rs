// Generated macro for other_38180 (other)
macro_rules! Depcrate_um_shellapiother_38180 {
() => {
// Module: crate::um::shellapi
// Provides: {"other_38180"}
// Dependencies: {}
extern "system" { pub fn Shell_NotifyIconA (dwMessage : DWORD , lpData : PNOTIFYICONDATAA ,) -> BOOL ; pub fn Shell_NotifyIconW (dwMessage : DWORD , lpData : PNOTIFYICONDATAW ,) -> BOOL ; pub fn Shell_NotifyIconGetRect (identifier : * const NOTIFYICONIDENTIFIER , iconLocation : * mut RECT ,) -> HRESULT ; }
};
}
