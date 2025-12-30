// Generated macro for macro_38990 (macro)
macro_rules! Depcrate_um_taskschdmacro_38990 {
() => {
// Module: crate::um::taskschd
// Provides: {"macro_38990"}
// Dependencies: {}
RIDL ! { # [uuid (0x5038fc98 , 0x82ff , 0x436d , 0x87 , 0x28 , 0xa5 , 0x12 , 0xa5 , 0x7c , 0x9d , 0xc1)] interface IWeeklyTrigger (IWeeklyTriggerVtbl) : ITrigger (ITriggerVtbl) { fn get_DaysOfWeek (pDays : * mut c_short ,) -> HRESULT , fn put_DaysOfWeek (pDays : c_short ,) -> HRESULT , fn get_WeeksInterval (pWeeks : * mut c_short ,) -> HRESULT , fn put_WeeksInterval (pWeeks : c_short ,) -> HRESULT , fn get_RandomDelay (pRandomDelay : * mut BSTR ,) -> HRESULT , fn put_RandomDelay (pRandomDelay : BSTR ,) -> HRESULT , } }
};
}
