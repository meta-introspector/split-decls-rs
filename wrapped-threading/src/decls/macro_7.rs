macro_rules! deps {
    () => {
        PTP_POOL!();
        BOOL!();
    };
}

macro_rules! macro_7 {
    () => {
        deps!();
        windows_link :: link ! ("kernel32.dll" "system" fn SetThreadpoolThreadMinimum (ptpp : PTP_POOL , cthrdmic : u32) -> BOOL) ;
    };
}

macro_7!();