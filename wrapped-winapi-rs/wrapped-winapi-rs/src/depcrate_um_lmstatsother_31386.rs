// Generated macro for other_31386 (other)
macro_rules! Depcrate_um_lmstatsother_31386 {
() => {
// Module: crate::um::lmstats
// Provides: {"other_31386"}
// Dependencies: {}
extern "system" { pub fn NetStatisticsGet (ServerName : LPCWSTR , Service : LPCWSTR , Level : DWORD , Options : DWORD , Buffer : * mut LPBYTE ,) -> NET_API_STATUS ; }
};
}
