macro_rules! deps {
    () => {
        SERVICE_STATUS_HANDLE!();
        PCWSTR!();
        LPHANDLER_FUNCTION_EX!();
    };
}

macro_rules! macro_0 {
    () => {
        deps!();
        windows_link :: link ! ("advapi32.dll" "system" fn RegisterServiceCtrlHandlerExW (lpservicename : PCWSTR , lphandlerproc : LPHANDLER_FUNCTION_EX , lpcontext : * const core :: ffi :: c_void) -> SERVICE_STATUS_HANDLE) ;
    };
}

macro_0!()