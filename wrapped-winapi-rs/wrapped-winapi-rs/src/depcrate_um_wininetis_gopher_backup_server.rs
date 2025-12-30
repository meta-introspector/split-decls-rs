// Generated macro for IS_GOPHER_BACKUP_SERVER (function)
macro_rules! Depcrate_um_wininetIS_GOPHER_BACKUP_SERVER {
() => {
// Module: crate::um::wininet
// Provides: {"IS_GOPHER_BACKUP_SERVER"}
// Dependencies: {}
# [inline] pub fn IS_GOPHER_BACKUP_SERVER (type_ : DWORD) -> BOOL { if (type_ & GOPHER_TYPE_REDUNDANT) != 0 { TRUE } else { FALSE } }
};
}
