// Generated macro for SyncReadAdapter (struct)
macro_rules! Depcrate_commonSyncReadAdapter {
() => {
// Module: crate::common
// Provides: {"SyncReadAdapter"}
// Dependencies: {}
# [doc = " An adapter that implements a [`Read`] interface for [`AsyncRead`] types and an"] # [doc = " associated [`Context`]."] # [doc = ""] # [doc = " Turns `Poll::Pending` into `WouldBlock`."] pub (crate) struct SyncReadAdapter < 'a , 'b , T > { pub (crate) io : & 'a mut T , pub (crate) cx : & 'a mut Context < 'b > , }
};
}
