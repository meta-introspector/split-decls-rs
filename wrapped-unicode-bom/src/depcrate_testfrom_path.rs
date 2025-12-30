// Generated macro for from_path (function)
macro_rules! Depcrate_testfrom_path {
() => {
// Module: crate::test
// Provides: {"from_path"}
// Dependencies: {}
# [test] fn from_path () -> Result < () , Error > { let bom : Bom = "fixtures/ascii.txt" . parse () ? ; assert_eq ! (bom , Bom :: Null) ; let bom : Bom = "fixtures/utf16-le.txt" . parse () ? ; assert_eq ! (bom , Bom :: Utf16Le) ; let bom : Bom = "fixtures/utf32-le.txt" . parse () ? ; assert_eq ! (bom , Bom :: Utf32Le) ; Ok (()) }
};
}
