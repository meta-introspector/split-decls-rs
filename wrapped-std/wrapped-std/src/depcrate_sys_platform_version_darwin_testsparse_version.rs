// Generated macro for parse_version (function)
macro_rules! Depcrate_sys_platform_version_darwin_testsparse_version {
() => {
// Module: crate::sys::platform_version::darwin::tests
// Provides: {"parse_version"}
// Dependencies: {}
# [test] fn parse_version () { # [track_caller] fn check (major : u16 , minor : u8 , patch : u8 , version : & str) { assert_eq ! (pack_os_version (major , minor , patch) , parse_os_version (version . as_bytes ()) . unwrap ()) } check (0 , 0 , 0 , "0") ; check (0 , 0 , 0 , "0.0.0") ; check (1 , 0 , 0 , "1") ; check (1 , 2 , 0 , "1.2") ; check (1 , 2 , 3 , "1.2.3") ; check (9999 , 99 , 99 , "9999.99.99") ; check (10 , 0 , 0 , "010") ; check (10 , 20 , 0 , "010.020") ; check (10 , 20 , 30 , "010.020.030") ; check (10000 , 100 , 100 , "000010000.00100.00100") ; assert ! (parse_os_version (b"1.2.3.4") . is_err ()) ; assert ! (parse_os_version (b"") . is_err ()) ; assert ! (parse_os_version (b"A.B") . is_err ()) ; assert ! (parse_os_version (b".") . is_err ()) ; assert ! (parse_os_version (b".1") . is_err ()) ; assert ! (parse_os_version (b"1.") . is_err ()) ; assert ! (parse_os_version (b"100000") . is_err ()) ; assert ! (parse_os_version (b"1.1000") . is_err ()) ; assert ! (parse_os_version (b"1.1.1000") . is_err ()) ; }
};
}
