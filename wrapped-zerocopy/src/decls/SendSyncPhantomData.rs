macro_rules! SendSyncPhantomData {
    () => {
        # [doc = " Like [`PhantomData`], but [`Send`] and [`Sync`] regardless of whether the"] # [doc = " wrapped `T` is."] pub (crate) struct SendSyncPhantomData < T : ? Sized > (PhantomData < T >) ;
    };
}

SendSyncPhantomData!()