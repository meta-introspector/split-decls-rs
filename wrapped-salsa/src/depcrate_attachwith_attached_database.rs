// Generated macro for with_attached_database (function)
macro_rules! Depcrate_attachwith_attached_database {
() => {
// Module: crate::attach
// Provides: {"with_attached_database"}
// Dependencies: {}
# [doc = " Access the \"attached\" database. Returns `None` if no database is attached."] # [doc = " Databases are attached with `attach_database`."] # [inline] pub fn with_attached_database < R > (op : impl FnOnce (& dyn Database) -> R) -> Option < R > { ATTACHED . with (# [inline] | a | a . with (op) ,) }
};
}
