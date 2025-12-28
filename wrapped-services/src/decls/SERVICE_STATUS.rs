macro_rules! deps {
    () => {
        ENUM_SERVICE_TYPE!();
        SERVICE_STATUS_CURRENT_STATE!();
    };
}

macro_rules! SERVICE_STATUS {
    () => {
        deps!();
        # [repr (C)] # [derive (Clone , Copy , Default)] pub struct SERVICE_STATUS { pub dwServiceType : ENUM_SERVICE_TYPE , pub dwCurrentState : SERVICE_STATUS_CURRENT_STATE , pub dwControlsAccepted : u32 , pub dwWin32ExitCode : u32 , pub dwServiceSpecificExitCode : u32 , pub dwCheckPoint : u32 , pub dwWaitHint : u32 , }
    };
}

SERVICE_STATUS!()