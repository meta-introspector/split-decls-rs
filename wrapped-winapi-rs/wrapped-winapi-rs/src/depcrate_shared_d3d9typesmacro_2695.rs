// Generated macro for macro_2695 (macro)
macro_rules! Depcrate_shared_d3d9typesmacro_2695 {
() => {
// Module: crate::shared::d3d9types
// Provides: {"macro_2695"}
// Dependencies: {}
STRUCT ! { # [cfg_attr (target_arch = "x86" , repr (packed))] struct D3DADAPTER_IDENTIFIER9 { Driver : [c_char ; MAX_DEVICE_IDENTIFIER_STRING] , Description : [c_char ; MAX_DEVICE_IDENTIFIER_STRING] , DeviceName : [c_char ; 32] , DriverVersion : LARGE_INTEGER , VendorId : DWORD , DeviceId : DWORD , SubSysId : DWORD , Revision : DWORD , DeviceIdentifier : GUID , WHQLLevel : DWORD , } }
};
}
