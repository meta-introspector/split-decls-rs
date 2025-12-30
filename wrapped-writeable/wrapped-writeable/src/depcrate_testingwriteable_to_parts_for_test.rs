// Generated macro for writeable_to_parts_for_test (function)
macro_rules! Depcrate_testingwriteable_to_parts_for_test {
() => {
// Module: crate::testing
// Provides: {"writeable_to_parts_for_test"}
// Dependencies: {}
pub fn writeable_to_parts_for_test < W : Writeable > (writeable : & W ,) -> (String , Vec < (usize , usize , Part) >) { let mut writer = TestWriter { string : alloc :: string :: String :: new () , parts : Vec :: new () , } ; # [expect (clippy :: expect_used)] writeable . write_to_parts (& mut writer) . expect ("String writer infallible") ; writer . finish () }
};
}
