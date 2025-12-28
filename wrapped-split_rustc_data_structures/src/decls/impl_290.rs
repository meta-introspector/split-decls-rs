macro_rules! deps {
    () => {
        IntoDynSyncSend!();
        DynSync!();
    };
}

macro_rules! impl_290 {
    () => {
        deps!();
        unsafe impl < T : ? Sized + PointeeSized + Sync > DynSync for IntoDynSyncSend < T > { }
    };
}

impl_290!()