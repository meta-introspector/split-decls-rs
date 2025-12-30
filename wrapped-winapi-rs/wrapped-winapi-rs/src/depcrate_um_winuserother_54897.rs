// Generated macro for other_54897 (other)
macro_rules! Depcrate_um_winuserother_54897 {
() => {
// Module: crate::um::winuser
// Provides: {"other_54897"}
// Dependencies: {}
extern "system" { pub fn CreateWindowStationA (lpwinsta : LPCSTR , dwFlags : DWORD , dwDesiredAccess : ACCESS_MASK , lpsa : LPSECURITY_ATTRIBUTES ,) -> HWINSTA ; pub fn CreateWindowStationW (lpwinsta : LPCWSTR , dwFlags : DWORD , dwDesiredAccess : ACCESS_MASK , lpsa : LPSECURITY_ATTRIBUTES ,) -> HWINSTA ; pub fn OpenWindowStationA (lpszWinSta : LPCSTR , fInherit : BOOL , dwDesiredAccess : ACCESS_MASK ,) -> HWINSTA ; pub fn OpenWindowStationW (lpszWinSta : LPCWSTR , fInherit : BOOL , dwDesiredAccess : ACCESS_MASK ,) -> HWINSTA ; pub fn EnumWindowStationsA (lpEnumFunc : WINSTAENUMPROCA , lParam : LPARAM ,) -> BOOL ; pub fn EnumWindowStationsW (lpEnumFunc : WINSTAENUMPROCW , lParam : LPARAM ,) -> BOOL ; pub fn CloseWindowStation (hWinSta : HWINSTA ,) -> BOOL ; pub fn SetProcessWindowStation (hWinSta : HWINSTA ,) -> BOOL ; pub fn GetProcessWindowStation () -> HWINSTA ; pub fn SetUserObjectSecurity (hObj : HANDLE , pSIRequested : PSECURITY_INFORMATION , pSID : PSECURITY_DESCRIPTOR ,) -> BOOL ; pub fn GetUserObjectSecurity (hObj : HANDLE , pSIRequested : PSECURITY_INFORMATION , pSID : PSECURITY_DESCRIPTOR , nLength : DWORD , lpnLengthNeeded : LPDWORD ,) -> BOOL ; }
};
}
