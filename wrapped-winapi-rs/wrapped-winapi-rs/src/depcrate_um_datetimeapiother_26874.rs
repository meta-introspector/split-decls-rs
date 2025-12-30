// Generated macro for other_26874 (other)
macro_rules! Depcrate_um_datetimeapiother_26874 {
() => {
// Module: crate::um::datetimeapi
// Provides: {"other_26874"}
// Dependencies: {}
extern "system" { pub fn GetDateFormatA (Locale : LCID , dwFlags : DWORD , lpDate : * const SYSTEMTIME , lpFormat : LPCSTR , lpDateStr : LPSTR , cchDate : c_int ,) -> c_int ; pub fn GetDateFormatW (Locale : LCID , dwFlags : DWORD , lpDate : * const SYSTEMTIME , lpFormat : LPCWSTR , lpDateStr : LPWSTR , cchDate : c_int ,) -> c_int ; pub fn GetTimeFormatA (Locale : LCID , dwFlags : DWORD , lpTime : * const SYSTEMTIME , lpFormat : LPCSTR , lpTimeStr : LPSTR , cchTime : c_int ,) -> c_int ; pub fn GetTimeFormatW (Locale : LCID , dwFlags : DWORD , lpTime : * const SYSTEMTIME , lpFormat : LPCWSTR , lpTimeStr : LPWSTR , cchTime : c_int ,) -> c_int ; pub fn GetTimeFormatEx (lpLocaleName : LPCWSTR , dwFlags : DWORD , lpTime : * const SYSTEMTIME , lpFormat : LPCWSTR , lpTimeStr : LPWSTR , cchTime : c_int ,) -> c_int ; pub fn GetDateFormatEx (lpLocaleName : LPCWSTR , dwFlags : DWORD , lpDate : * const SYSTEMTIME , lpFormat : LPCWSTR , lpDateStr : LPWSTR , cchDate : c_int , lpCalendar : LPCWSTR ,) -> c_int ; }
};
}
