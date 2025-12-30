// Generated macro for other_37699 (other)
macro_rules! Depcrate_um_setupapiother_37699 {
() => {
// Module: crate::um::setupapi
// Provides: {"other_37699"}
// Dependencies: {}
extern "system" { pub fn SetupSetDirectoryIdA (InfHandle : HINF , Id : DWORD , Directory : PCSTR ,) -> BOOL ; pub fn SetupSetDirectoryIdW (InfHandle : HINF , Id : DWORD , Directory : PCWSTR ,) -> BOOL ; pub fn SetupSetDirectoryIdExA (InfHandle : HINF , Id : DWORD , Directory : PCSTR , Flags : DWORD , Reserved1 : DWORD , Reserved2 : PVOID ,) -> BOOL ; pub fn SetupSetDirectoryIdExW (InfHandle : HINF , Id : DWORD , Directory : PCWSTR , Flags : DWORD , Reserved1 : DWORD , Reserved2 : PVOID ,) -> BOOL ; }
};
}
