macro_rules! deps {
    () => {
        PTP_POOL!();
    };
}

macro_rules! macro_3 {
    () => {
        deps!();
        windows_link :: link ! ("kernel32.dll" "system" fn CreateThreadpool (reserved : * const core :: ffi :: c_void) -> PTP_POOL) ;
    };
}

macro_3!();