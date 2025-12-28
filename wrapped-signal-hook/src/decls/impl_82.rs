macro_rules! deps {
    () => {
        Process!();
    };
}

macro_rules! impl_82 {
    () => {
        deps!();
        impl Process { # [doc = "\n     * Extract the process information.\n     *\n     * # Safety\n     *\n     * The `info` must have a `si_code` corresponding to some situation that has the `si_pid`\n     * and `si_uid` filled in.\n     "] unsafe fn extract (info : & siginfo_t) -> Self { Self { pid : sighook_signal_pid (info) , uid : sighook_signal_uid (info) , } } }
    };
}

impl_82!();