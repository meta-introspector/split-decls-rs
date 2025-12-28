macro_rules! deps {
    () => {
        WakeMethod!();
    };
}

macro_rules! WakeFd {
    () => {
        deps!();
        struct WakeFd { fd : RawFd , method : WakeMethod , }
    };
}

WakeFd!();