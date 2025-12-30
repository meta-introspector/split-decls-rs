// Generated macro for ensure_file_contents (function)
macro_rules! Depcrateensure_file_contents {
() => {
// Module: crate
// Provides: {"ensure_file_contents"}
// Dependencies: {}
# [doc = " Checks that the `file` has the specified `contents`. If that is not the"] # [doc = " case, updates the file and then fails the test."] # [track_caller] pub fn ensure_file_contents (file : & Path , contents : & str) { if let Err (()) = try_ensure_file_contents (file , contents) { panic ! ("Some files were not up-to-date") ; } }
};
}
