macro_rules! deps {
    () => {
        PTP_POOL!();
    };
}

macro_rules! macro_0 {
    () => {
        deps!();
        windows_link :: link ! ("kernel32.dll" "system" fn CloseThreadpool (ptpp : PTP_POOL)) ;
    };
}

macro_0!()