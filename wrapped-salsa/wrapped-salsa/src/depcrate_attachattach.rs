// Generated macro for attach (function)
macro_rules! Depcrate_attachattach {
() => {
// Module: crate::attach
// Provides: {"attach"}
// Dependencies: {}
# [doc = " Attach the database to the current thread and execute `op`."] # [doc = " Panics if a different database has already been attached."] # [inline] pub fn attach < R , Db > (db : & Db , op : impl FnOnce () -> R) -> R where Db : ? Sized + Database , { ATTACHED . with (# [inline] | a | a . attach (db , op) ,) }
};
}
