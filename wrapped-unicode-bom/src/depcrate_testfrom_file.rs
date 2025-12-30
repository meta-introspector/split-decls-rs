// Generated macro for from_file (function)
macro_rules! Depcrate_testfrom_file {
() => {
// Module: crate::test
// Provides: {"from_file"}
// Dependencies: {}
# [test] fn from_file () { let mut file = File :: open ("fixtures/ascii.txt") . unwrap () ; assert_eq ! (Bom :: from (& mut file) , Bom :: Null) ; let mut file = File :: open ("fixtures/utf16-le.txt") . unwrap () ; assert_eq ! (Bom :: from (& mut file) , Bom :: Utf16Le) ; let mut file = File :: open ("fixtures/utf32-le.txt") . unwrap () ; assert_eq ! (Bom :: from (& mut file) , Bom :: Utf32Le) ; }
};
}
