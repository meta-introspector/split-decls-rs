// Generated macro for tests (module)
macro_rules! Depcrate_serial_file_locktests {
() => {
// Module: crate::serial_file_lock
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use std :: panic ; use fslock :: LockFile ; use super :: fs_serial_core ; use crate :: file_lock :: path_for_name ; # [test] fn test_serial () { fs_serial_core (vec ! ["test"] , None , | | { }) ; } # [test] fn unlock_on_assert_sync_without_return () { let lock_path = path_for_name ("serial_unlock_on_assert_sync_without_return") ; let _ = panic :: catch_unwind (| | { fs_serial_core (vec ! ["serial_unlock_on_assert_sync_without_return"] , Some (& lock_path) , | | { assert ! (false) ; } ,) }) ; let mut lockfile = LockFile :: open (& lock_path) . unwrap () ; assert ! (lockfile . try_lock () . unwrap ()) ; } }
};
}
