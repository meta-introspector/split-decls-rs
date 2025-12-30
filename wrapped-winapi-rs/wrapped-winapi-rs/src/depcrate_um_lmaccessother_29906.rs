// Generated macro for other_29906 (other)
macro_rules! Depcrate_um_lmaccessother_29906 {
() => {
// Module: crate::um::lmaccess
// Provides: {"other_29906"}
// Dependencies: {}
extern "system" { pub fn NetQueryDisplayInformation (ServerName : LPCWSTR , Level : DWORD , Index : DWORD , EntriesRequested : DWORD , PreferredMaximumLength : DWORD , ReturnedEntryCount : LPDWORD , SortedBuffer : * mut PVOID ,) -> NET_API_STATUS ; pub fn NetGetDisplayInformationIndex (ServerName : LPCWSTR , Level : DWORD , Prefix : LPCWSTR , Index : LPDWORD ,) -> NET_API_STATUS ; }
};
}
