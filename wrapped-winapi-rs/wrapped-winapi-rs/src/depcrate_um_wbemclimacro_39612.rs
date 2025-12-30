// Generated macro for macro_39612 (macro)
macro_rules! Depcrate_um_wbemclimacro_39612 {
() => {
// Module: crate::um::wbemcli
// Provides: {"macro_39612"}
// Dependencies: {}
RIDL ! { # [uuid (0xc49e32c7 , 0xbc8b , 0x11d2 , 0x85 , 0xd4 , 0x00 , 0x10 , 0x5a , 0x1f , 0x83 , 0x04)] interface IWbemBackupRestore (IWbemBackupRestoreVtbl) : IUnknown (IUnknownVtbl) { fn Backup (strBackupToFile : LPCWSTR , lFlags : c_long ,) -> HRESULT , fn Restore (strRestoreFromFile : LPCWSTR , lFlags : c_long ,) -> HRESULT , } }
};
}
