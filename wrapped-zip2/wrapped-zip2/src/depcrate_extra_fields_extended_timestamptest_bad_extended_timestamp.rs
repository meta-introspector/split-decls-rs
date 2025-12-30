// Generated macro for test_bad_extended_timestamp (function)
macro_rules! Depcrate_extra_fields_extended_timestamptest_bad_extended_timestamp {
() => {
// Module: crate::extra_fields::extended_timestamp
// Provides: {"test_bad_extended_timestamp"}
// Dependencies: {}
# [test] # [doc = " Ensure we don't panic or read garbage data if the field body is empty"] pub fn test_bad_extended_timestamp () -> ZipResult < () > { use crate :: ZipArchive ; use std :: io :: Cursor ; assert ! (ZipArchive :: new (Cursor :: new (include_bytes ! ("../../tests/data/extended_timestamp_bad.zip"))) . is_err ()) ; Ok (()) }
};
}
