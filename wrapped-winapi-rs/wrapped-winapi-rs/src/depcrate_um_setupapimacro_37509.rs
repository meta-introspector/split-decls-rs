// Generated macro for macro_37509 (macro)
macro_rules! Depcrate_um_setupapimacro_37509 {
() => {
// Module: crate::um::setupapi
// Provides: {"macro_37509"}
// Dependencies: {}
STRUCT ! { # [cfg_attr (target_arch = "x86" , repr (packed))] struct SP_DRVINFO_DETAIL_DATA_W { cbSize : DWORD , InfDate : FILETIME , CompatIDsOffset : DWORD , CompatIDsLength : DWORD , Reserved : ULONG_PTR , SectionName : [WCHAR ; LINE_LEN] , InfFileName : [WCHAR ; MAX_PATH] , DrvDescription : [WCHAR ; LINE_LEN] , HardwareID : [WCHAR ; ANYSIZE_ARRAY] , } }
};
}
