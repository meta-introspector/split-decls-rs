// Generated macro for tests (module)
macro_rules! Depcrate_ideographtests {
() => {
// Module: crate::ideograph
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: ideograph_name ; # [test] fn name () { assert_eq ! (ideograph_name (0x4E00) . unwrap () , "CJK UNIFIED IDEOGRAPH-4E00") ; assert_eq ! (ideograph_name (0x9FD5) . unwrap () , "CJK UNIFIED IDEOGRAPH-9FD5") ; assert_eq ! (ideograph_name (0x17000) . unwrap () , "TANGUT IDEOGRAPH-17000") ; assert_eq ! (ideograph_name (0xF900) . unwrap () , "CJK COMPATIBILITY IDEOGRAPH-F900") ; } # [test] fn invalid () { assert ! (ideograph_name (0) . is_none ()) ; } }
};
}
