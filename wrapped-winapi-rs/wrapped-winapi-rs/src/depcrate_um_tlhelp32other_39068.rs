// Generated macro for other_39068 (other)
macro_rules! Depcrate_um_tlhelp32other_39068 {
() => {
// Module: crate::um::tlhelp32
// Provides: {"other_39068"}
// Dependencies: {}
extern "system" { pub fn Process32FirstW (hSnapshot : HANDLE , lppe : LPPROCESSENTRY32W ,) -> BOOL ; pub fn Process32NextW (hSnapshot : HANDLE , lppe : LPPROCESSENTRY32W ,) -> BOOL ; }
};
}
