macro_rules! deps {
    () => {
        PTP_CLEANUP_GROUP!();
    };
}

macro_rules! macro_4 {
    () => {
        deps!();
        windows_link :: link ! ("kernel32.dll" "system" fn CreateThreadpoolCleanupGroup () -> PTP_CLEANUP_GROUP) ;
    };
}

macro_4!();