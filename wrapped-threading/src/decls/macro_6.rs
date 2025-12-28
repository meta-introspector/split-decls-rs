macro_rules! deps {
    () => {
        PTP_POOL!();
    };
}

macro_rules! macro_6 {
    () => {
        deps!();
        windows_link :: link ! ("kernel32.dll" "system" fn SetThreadpoolThreadMaximum (ptpp : PTP_POOL , cthrdmost : u32)) ;
    };
}

macro_6!()