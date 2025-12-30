// Generated macro for attach_allow_change (function)
macro_rules! Depcrate_attachattach_allow_change {
() => {
// Module: crate::attach
// Provides: {"attach_allow_change"}
// Dependencies: {}
# [doc = " Attach the database to the current thread and execute `op`."] # [doc = " Allows a different database than currently attached. The original database"] # [doc = " will be restored on return."] # [doc = ""] # [doc = " **Note:** Switching databases can cause bugs. If you do not intend to switch"] # [doc = " databases, prefer [`attach`] which will panic if you accidentally do."] # [inline] pub fn attach_allow_change < R , Db > (db : & Db , op : impl FnOnce () -> R) -> R where Db : ? Sized + Database , { ATTACHED . with (# [inline] | a | a . attach_allow_change (db , op) ,) }
};
}
