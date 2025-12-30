// Generated macro for macro_20594 (macro)
macro_rules! Depcrate_um_cfgmgr32macro_20594 {
() => {
// Module: crate::um::cfgmgr32
// Provides: {"macro_20594"}
// Dependencies: {}
STRUCT ! { # [repr (packed)] struct PCCARD_DES { PCD_Count : DWORD , PCD_Type : DWORD , PCD_Flags : DWORD , PCD_ConfigIndex : BYTE , PCD_Reserved : [BYTE ; 3] , PCD_MemoryCardBase1 : DWORD , PCD_MemoryCardBase2 : DWORD , PCD_MemoryCardBase : [DWORD ; PCD_MAX_MEMORY] , PCD_MemoryFlags : [WORD ; PCD_MAX_MEMORY] , PCD_IoFlags : [BYTE ; PCD_MAX_IO] , } }
};
}
