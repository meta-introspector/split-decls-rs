// Generated macro for SCPreferencesCallBack (type)
macro_rules! Depcrate_preferencesSCPreferencesCallBack {
() => {
// Module: crate::preferences
// Provides: {"SCPreferencesCallBack"}
// Dependencies: {}
pub type SCPreferencesCallBack = Option < unsafe extern "C" fn (prefs : SCPreferencesRef , notificationType : SCPreferencesNotification , info : * mut :: core :: ffi :: c_void ,) , > ;
};
}
