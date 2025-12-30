// Generated macro for try_writeable_to_parts_for_test (function)
macro_rules! Depcrate_testingtry_writeable_to_parts_for_test {
() => {
// Module: crate::testing
// Provides: {"try_writeable_to_parts_for_test"}
// Dependencies: {}
# [expect (clippy :: type_complexity)] pub fn try_writeable_to_parts_for_test < W : TryWriteable > (writeable : & W ,) -> (String , Vec < (usize , usize , Part) > , Option < W :: Error >) { let mut writer = TestWriter { string : alloc :: string :: String :: new () , parts : Vec :: new () , } ; # [expect (clippy :: expect_used)] let result = writeable . try_write_to_parts (& mut writer) . expect ("String writer infallible") ; let (actual_str , actual_parts) = writer . finish () ; (actual_str , actual_parts , result . err ()) }
};
}
