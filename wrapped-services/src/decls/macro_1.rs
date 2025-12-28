macro_rules! deps {
    () => {
        BOOL!();
        SERVICE_STATUS!();
        SERVICE_STATUS_HANDLE!();
    };
}

macro_rules! macro_1 {
    () => {
        deps!();
        windows_link :: link ! ("advapi32.dll" "system" fn SetServiceStatus (hservicestatus : SERVICE_STATUS_HANDLE , lpservicestatus : * const SERVICE_STATUS) -> BOOL) ;
    };
}

macro_1!()