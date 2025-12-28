macro_rules! deps {
    () => {
        SERVICE_STATUS_HANDLE!();
        SERVICE_STATUS!();
        BOOL!();
    };
}

macro_rules! macro_1 {
    () => {
        deps!();
        windows_link :: link ! ("advapi32.dll" "system" fn SetServiceStatus (hservicestatus : SERVICE_STATUS_HANDLE , lpservicestatus : * const SERVICE_STATUS) -> BOOL) ;
    };
}

macro_1!();