// Generated macro for other_39072 (other)
macro_rules! Depcrate_um_tlhelp32other_39072 {
() => {
// Module: crate::um::tlhelp32
// Provides: {"other_39072"}
// Dependencies: {}
extern "system" { pub fn Process32First (hSnapshot : HANDLE , lppe : LPPROCESSENTRY32 ,) -> BOOL ; pub fn Process32Next (hSnapshot : HANDLE , lppe : LPPROCESSENTRY32 ,) -> BOOL ; }
};
}
