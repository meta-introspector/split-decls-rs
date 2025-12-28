macro_rules! IntoDynSyncSend {
    () => {
        # [derive (Copy , Clone)] pub struct IntoDynSyncSend < T : ? Sized + PointeeSized > (pub T) ;
    };
}

IntoDynSyncSend!()