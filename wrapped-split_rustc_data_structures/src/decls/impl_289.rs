macro_rules! deps {
    () => {
        IntoDynSyncSend!();
        DynSend!();
    };
}

macro_rules! impl_289 {
    () => {
        deps!();
        unsafe impl < T : ? Sized + PointeeSized + Send > DynSend for IntoDynSyncSend < T > { }
    };
}

impl_289!();