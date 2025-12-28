macro_rules! deps {
    () => {
        ThreadPoolBuildError!();
    };
}

macro_rules! check_error_send_sync {
    () => {
        deps!();
        # [test] fn check_error_send_sync () { _send_sync :: < ThreadPoolBuildError > () ; }
    };
}

check_error_send_sync!()