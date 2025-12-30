// Generated macro for other_34260 (other)
macro_rules! Depcrate_um_physicalmonitorenumerationapiother_34260 {
() => {
// Module: crate::um::physicalmonitorenumerationapi
// Provides: {"other_34260"}
// Dependencies: {}
extern "system" { pub fn GetNumberOfPhysicalMonitorsFromHMONITOR (hMonitor : HMONITOR , pdwNumberOfPhysicalMonitor : LPDWORD ,) -> _BOOL ; pub fn GetNumberOfPhysicalMonitorsFromIDirect3DDevice9 (pDirect3DDevice9 : * mut IDirect3DDevice9 , pdwNumberOfPhysicalMonitor : LPDWORD ,) -> HRESULT ; pub fn GetPhysicalMonitorsFromHMONITOR (hMonitor : HMONITOR , dwPhysicalMonitorArraySize : DWORD , pPhysicalMonitorArray : LPPHYSICAL_MONITOR ,) -> _BOOL ; pub fn GetPhysicalMonitorsFromIDirect3DDevice9 (pDirect3DDevice9 : IDirect3DDevice9 , dwPhysicalMonitorArraySize : DWORD , pPhysicalMonitorArray : LPPHYSICAL_MONITOR ,) -> HRESULT ; pub fn DestroyPhysicalMonitor (hMonitor : HANDLE ,) -> _BOOL ; pub fn DestroyPhysicalMonitors (dwPhysicalMonitorArraySize : DWORD , pPhysicalMonitorArray : LPPHYSICAL_MONITOR ,) -> _BOOL ; }
};
}
