// Generated macro for other_39027 (other)
macro_rules! Depcrate_um_timeapiother_39027 {
() => {
// Module: crate::um::timeapi
// Provides: {"other_39027"}
// Dependencies: {}
extern "system" { pub fn timeGetTime () -> DWORD ; pub fn timeGetDevCaps (ptc : LPTIMECAPS , cbtc : UINT ,) -> MMRESULT ; pub fn timeBeginPeriod (uPeriod : UINT ,) -> MMRESULT ; pub fn timeEndPeriod (uPeriod : UINT ,) -> MMRESULT ; }
};
}
