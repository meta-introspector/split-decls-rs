macro_rules! deps {
    () => {
        ThreadPoolBuildError!();
    };
}

macro_rules! _send_sync {
    () => {
        deps!();
        # [doc = " Helper used by check_error_send_sync to ensure ThreadPoolBuildError is Send + Sync"] fn _send_sync < T : Send + Sync > () { }
    };
}

_send_sync!();