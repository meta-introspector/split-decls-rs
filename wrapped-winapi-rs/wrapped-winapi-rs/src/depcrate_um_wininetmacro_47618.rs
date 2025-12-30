// Generated macro for macro_47618 (macro)
macro_rules! Depcrate_um_wininetmacro_47618 {
() => {
// Module: crate::um::wininet
// Provides: {"macro_47618"}
// Dependencies: {}
STRUCT ! { struct AutoProxyHelperVtbl { IsResolvable : Option < unsafe extern "system" fn (lpszHost : LPSTR ,) -> BOOL >, GetIPAddress : Option < unsafe extern "system" fn (lpszIPAddress : LPSTR , lpdwIPAddressSize : LPDWORD ,) -> DWORD >, ResolveHostName : Option < unsafe extern "system" fn (lpszHostName : LPSTR , lpszIPAddress : LPSTR , lpdwIPAddressSize : LPDWORD ,) -> DWORD >, IsInNet : Option < unsafe extern "system" fn (lpszIPAddress : LPSTR , lpszDest : LPSTR , lpszMask : LPSTR ,) -> BOOL >, IsResolvableEx : Option < unsafe extern "system" fn (lpszHost : LPSTR ,) -> BOOL >, GetIPAddressEx : Option < unsafe extern "system" fn (lpszIPAddress : LPSTR , lpdwIPAddressSize : LPDWORD ,) -> DWORD >, ResolveHostNameEx : Option < unsafe extern "system" fn (lpszHostName : LPSTR , lpszIPAddress : LPSTR , lpdwIPAddressSize : LPDWORD ,) -> DWORD >, IsInNetEx : Option < unsafe extern "system" fn (lpszIPAddress : LPSTR , lpszIPPrefix : LPSTR ,) -> BOOL >, SortIpList : Option < unsafe extern "system" fn (lpszIPAddressList : LPSTR , lpszIPSortedList : LPSTR , lpdwIPSortedListSize : LPDWORD ,) -> DWORD >, } }
};
}
