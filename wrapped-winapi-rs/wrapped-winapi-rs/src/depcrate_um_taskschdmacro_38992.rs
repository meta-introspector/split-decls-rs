// Generated macro for macro_38992 (macro)
macro_rules! Depcrate_um_taskschdmacro_38992 {
() => {
// Module: crate::um::taskschd
// Provides: {"macro_38992"}
// Dependencies: {}
RIDL ! { # [uuid (0x77d025a3 , 0x90fa , 0x43aa , 0xb5 , 0x2e , 0xcd , 0xa5 , 0x49 , 0x9b , 0x94 , 0x6a)] interface IMonthlyDOWTrigger (IMonthlyDOWTriggerVtbl) : ITrigger (ITriggerVtbl) { fn get_DaysOfWeek (pDays : * mut c_short ,) -> HRESULT , fn put_DaysOfWeek (pDays : c_short ,) -> HRESULT , fn get_WeeksOfMonth (pWeeks : * mut c_short ,) -> HRESULT , fn put_WeeksOfMonth (pWeeks : c_short ,) -> HRESULT , fn get_MonthsOfYear (pMonths : * mut c_short ,) -> HRESULT , fn put_MonthsOfYear (pMonths : c_short ,) -> HRESULT , fn get_RunOnLastWeekOfMonth (pLastWeek : * mut VARIANT_BOOL ,) -> HRESULT , fn put_RunOnLastWeekOfMonth (pLastWeek : VARIANT_BOOL ,) -> HRESULT , fn get_RandomDelay (pRandomDelay : * mut BSTR ,) -> HRESULT , fn put_RandomDelay (pRandomDelay : BSTR ,) -> HRESULT , } }
};
}
