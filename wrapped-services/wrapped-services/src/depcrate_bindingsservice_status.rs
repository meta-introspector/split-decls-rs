// Generated macro for SERVICE_STATUS (struct)
macro_rules! Depcrate_bindingsSERVICE_STATUS {
() => {
// Module: crate::bindings
// Provides: {"SERVICE_STATUS"}
// Dependencies: {}
# [repr (C)] # [derive (Clone , Copy , Default)] pub struct SERVICE_STATUS { pub dwServiceType : ENUM_SERVICE_TYPE , pub dwCurrentState : SERVICE_STATUS_CURRENT_STATE , pub dwControlsAccepted : u32 , pub dwWin32ExitCode : u32 , pub dwServiceSpecificExitCode : u32 , pub dwCheckPoint : u32 , pub dwWaitHint : u32 , }
};
}
