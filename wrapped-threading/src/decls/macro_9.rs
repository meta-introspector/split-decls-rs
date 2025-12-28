macro_rules! deps {
    () => {
        BOOL!();
        TP_CALLBACK_ENVIRON_V3!();
        PTP_SIMPLE_CALLBACK!();
    };
}

macro_rules! macro_9 {
    () => {
        deps!();
        windows_link :: link ! ("kernel32.dll" "system" fn TrySubmitThreadpoolCallback (pfns : PTP_SIMPLE_CALLBACK , pv : * mut core :: ffi :: c_void , pcbe : * const TP_CALLBACK_ENVIRON_V3) -> BOOL) ;
    };
}

macro_9!();