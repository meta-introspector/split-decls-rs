// Generated macro for macro_38991 (macro)
macro_rules! Depcrate_um_taskschdmacro_38991 {
() => {
// Module: crate::um::taskschd
// Provides: {"macro_38991"}
// Dependencies: {}
RIDL ! { # [uuid (0x97c45ef1 , 0x6b02 , 0x4a1a , 0x9c , 0x0e , 0x1e , 0xbf , 0xba , 0x15 , 0x00 , 0xac)] interface IMonthlyTrigger (IMonthlyTriggerVtbl) : ITrigger (ITriggerVtbl) { fn get_DaysOfMonth (pDays : * mut c_long ,) -> HRESULT , fn put_DaysOfMonth (pDays : c_long ,) -> HRESULT , fn get_MonthsOfYear (pMonths : * mut c_short ,) -> HRESULT , fn put_MonthsOfYear (pMonths : c_short ,) -> HRESULT , fn get_RunOnLastDayOfMonth (pLastDay : * mut VARIANT_BOOL ,) -> HRESULT , fn put_RunOnLastDayOfMonth (pLastDay : VARIANT_BOOL ,) -> HRESULT , fn get_RandomDelay (pRandomDelay : * mut BSTR ,) -> HRESULT , fn put_RandomDelay (pRandomDelay : BSTR ,) -> HRESULT , } }
};
}
