// Generated macro for tests (module)
macro_rules! Depcrate_reporttests {
() => {
// Module: crate::report
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use crate :: ErrorReportingUtf8Chars ; # [test] fn test_size () { assert_eq ! (core :: mem :: size_of ::< Option << ErrorReportingUtf8Chars <'_ > as Iterator >:: Item >> () , core :: mem :: size_of ::< Option < char >> ()) ; } # [test] fn test_eq () { let a : < ErrorReportingUtf8Chars < '_ > as Iterator > :: Item = Ok ('a') ; let a_again : < ErrorReportingUtf8Chars < '_ > as Iterator > :: Item = Ok ('a') ; assert_eq ! (a , a_again) ; } }
};
}
