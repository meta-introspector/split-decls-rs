// Generated macro for other_31744 (other)
macro_rules! Depcrate_um_lowlevelmonitorconfigurationapiother_31744 {
() => {
// Module: crate::um::lowlevelmonitorconfigurationapi
// Provides: {"other_31744"}
// Dependencies: {}
extern "system" { pub fn GetVCPFeatureAndVCPFeatureReply (hMonitor : HANDLE , bVCPCode : BYTE , pvct : LPMC_VCP_CODE_TYPE , pdwCurrentValue : LPDWORD , pdwMaximumValue : LPDWORD ,) -> _BOOL ; pub fn SetVCPFeature (hMonitor : HANDLE , bVCPCode : BYTE , dwNewValue : DWORD ,) -> _BOOL ; pub fn SaveCurrentSettings (hMonitor : HANDLE ,) -> _BOOL ; pub fn GetCapabilitiesStringLength (hMonitor : HANDLE , pdwCapabilitiesStringLengthInCharacters : LPDWORD ,) -> _BOOL ; pub fn CapabilitiesRequestAndCapabilitiesReply (hMonitor : HANDLE , pszASCIICapabilitiesString : LPSTR , dwCapabilitiesStringLengthInCharacters : DWORD ,) -> _BOOL ; pub fn GetTimingReport (hMonitor : HANDLE , pmtrMonitorTimingReport : LPMC_TIMING_REPORT ,) -> _BOOL ; }
};
}
