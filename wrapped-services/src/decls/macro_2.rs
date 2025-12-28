macro_rules! deps {
    () => {
        SERVICE_TABLE_ENTRYW!();
        BOOL!();
    };
}

macro_rules! macro_2 {
    () => {
        deps!();
        windows_link :: link ! ("advapi32.dll" "system" fn StartServiceCtrlDispatcherW (lpservicestarttable : * const SERVICE_TABLE_ENTRYW) -> BOOL) ;
    };
}

macro_2!()