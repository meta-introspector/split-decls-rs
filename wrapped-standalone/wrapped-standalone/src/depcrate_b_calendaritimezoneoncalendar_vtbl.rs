// Generated macro for ITimeZoneOnCalendar_Vtbl (struct)
macro_rules! Depcrate_b_calendarITimeZoneOnCalendar_Vtbl {
() => {
// Module: crate::b_calendar
// Provides: {"ITimeZoneOnCalendar_Vtbl"}
// Dependencies: {}
# [repr (C)] # [doc (hidden)] pub struct ITimeZoneOnCalendar_Vtbl { pub base__ : windows_core :: IInspectable_Vtbl , pub GetTimeZone : unsafe extern "system" fn (* mut core :: ffi :: c_void , * mut * mut core :: ffi :: c_void ,) -> windows_core :: HRESULT , pub ChangeTimeZone : unsafe extern "system" fn (* mut core :: ffi :: c_void , * mut core :: ffi :: c_void ,) -> windows_core :: HRESULT , pub TimeZoneAsFullString : unsafe extern "system" fn (* mut core :: ffi :: c_void , * mut * mut core :: ffi :: c_void ,) -> windows_core :: HRESULT , pub TimeZoneAsString : unsafe extern "system" fn (* mut core :: ffi :: c_void , i32 , * mut * mut core :: ffi :: c_void ,) -> windows_core :: HRESULT , }
};
}
