macro_rules! deps {
    () => {
        PTP_CLEANUP_GROUP!();
        BOOL!();
    };
}

macro_rules! macro_2 {
    () => {
        deps!();
        windows_link :: link ! ("kernel32.dll" "system" fn CloseThreadpoolCleanupGroupMembers (ptpcg : PTP_CLEANUP_GROUP , fcancelpendingcallbacks : BOOL , pvcleanupcontext : * mut core :: ffi :: c_void)) ;
    };
}

macro_2!()