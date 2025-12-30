// Generated macro for macro_39003 (macro)
macro_rules! Depcrate_um_taskschdmacro_39003 {
() => {
// Module: crate::um::taskschd
// Provides: {"macro_39003"}
// Dependencies: {}
RIDL ! { # [uuid (0x0ad9d0d7 , 0x0c7f , 0x4ebb , 0x9a , 0x5f , 0xd1 , 0xc6 , 0x48 , 0xdc , 0xa5 , 0x28)] interface ITaskSettings3 (ITaskSettings3Vtbl) : ITaskSettings (ITaskSettingsVtbl) { fn get_DisallowStartOnRemoteAppSession (pDisallowStart : * mut VARIANT_BOOL ,) -> HRESULT , fn put_DisallowStartOnRemoteAppSession (pDisallowStart : VARIANT_BOOL ,) -> HRESULT , fn get_UseUnifiedSchedulingEngine (pUseUnifiedEngine : * mut VARIANT_BOOL ,) -> HRESULT , fn put_UseUnifiedSchedulingEngine (pUseUnifiedEngine : VARIANT_BOOL ,) -> HRESULT , fn get_MaintenanceSettings (ppMaintenanceSettings : * mut * mut IMaintenanceSettings ,) -> HRESULT , fn put_MaintenanceSettings (ppMaintenanceSettings : * const IMaintenanceSettings ,) -> HRESULT , fn CreateMaintenanceSettings (ppMaintenanceSettings : * mut * mut IMaintenanceSettings ,) -> HRESULT , fn get_Volatile (pVolatile : * mut VARIANT_BOOL ,) -> HRESULT , fn put_Volatile (pVolatile : VARIANT_BOOL ,) -> HRESULT , } }
};
}
