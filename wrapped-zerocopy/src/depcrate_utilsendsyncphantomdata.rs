// Generated macro for SendSyncPhantomData (struct)
macro_rules! Depcrate_utilSendSyncPhantomData {
() => {
// Module: crate::util
// Provides: {"SendSyncPhantomData"}
// Dependencies: {}
# [doc = " Like [`PhantomData`], but [`Send`] and [`Sync`] regardless of whether the"] # [doc = " wrapped `T` is."] pub (crate) struct SendSyncPhantomData < T : ? Sized > (PhantomData < T >) ;
};
}
