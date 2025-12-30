// Generated macro for SyncWriteAdapter (struct)
macro_rules! Depcrate_commonSyncWriteAdapter {
() => {
// Module: crate::common
// Provides: {"SyncWriteAdapter"}
// Dependencies: {}
# [doc = " An adapter that implements a [`Write`] interface for [`AsyncWrite`] types and an"] # [doc = " associated [`Context`]."] # [doc = ""] # [doc = " Turns `Poll::Pending` into `WouldBlock`."] pub (crate) struct SyncWriteAdapter < 'a , 'b , T > { pub (crate) io : & 'a mut T , pub (crate) cx : & 'a mut Context < 'b > , }
};
}
