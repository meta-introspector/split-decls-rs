macro_rules! deps {
    () => {
        PTP_CLEANUP_GROUP!();
    };
}

macro_rules! macro_1 {
    () => {
        deps!();
        windows_link :: link ! ("kernel32.dll" "system" fn CloseThreadpoolCleanupGroup (ptpcg : PTP_CLEANUP_GROUP)) ;
    };
}

macro_1!();