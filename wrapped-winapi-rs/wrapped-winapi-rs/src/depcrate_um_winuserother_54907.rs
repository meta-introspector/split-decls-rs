// Generated macro for other_54907 (other)
macro_rules! Depcrate_um_winuserother_54907 {
() => {
// Module: crate::um::winuser
// Provides: {"other_54907"}
// Dependencies: {}
extern "system" { pub fn GetUserObjectInformationA (hObj : HANDLE , nIndex : c_int , pvInfo : PVOID , nLength : DWORD , lpnLengthNeeded : LPDWORD ,) -> BOOL ; pub fn GetUserObjectInformationW (hObj : HANDLE , nIndex : c_int , pvInfo : PVOID , nLength : DWORD , lpnLengthNeeded : LPDWORD ,) -> BOOL ; pub fn SetUserObjectInformationA (hObj : HANDLE , nIndex : c_int , pvInfo : PVOID , nLength : DWORD ,) -> BOOL ; pub fn SetUserObjectInformationW (hObj : HANDLE , nIndex : c_int , pvInfo : PVOID , nLength : DWORD ,) -> BOOL ; }
};
}
