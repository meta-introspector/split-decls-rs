macro_rules! deps {
    () => {
        WorkerLocal!();
    };
}

macro_rules! impl_290 {
    () => {
        deps!();
        # [doc = " We prevent concurrent access to the underlying value in the"] # [doc = " Deref impl, thus any values safe to send across threads can"] # [doc = " be used with WorkerLocal."] unsafe impl < T : Send > Sync for WorkerLocal < T > { }
    };
}

impl_290!();