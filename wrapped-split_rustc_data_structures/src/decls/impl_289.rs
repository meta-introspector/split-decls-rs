macro_rules! deps {
    () => {
        DynSend!();
        IntoDynSyncSend!();
    };
}

macro_rules! impl_289 {
    () => {
        deps!();
        unsafe impl < T : ? Sized + PointeeSized + Send > DynSend for IntoDynSyncSend < T > { }
    };
}

impl_289!()