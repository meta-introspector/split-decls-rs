// Generated macro for filter_tests (function)
macro_rules! Depcratefilter_tests {
() => {
// Module: crate
// Provides: {"filter_tests"}
// Dependencies: {}
pub fn filter_tests (opts : & TestOpts , tests : Vec < TestDescAndFn >) -> Vec < TestDescAndFn > { let mut filtered = tests ; let matches_filter = | test : & TestDescAndFn , filter : & str | { let test_name = test . desc . name . as_slice () ; match opts . filter_exact { true => test_name == filter , false => test_name . contains (filter) , } } ; if ! opts . filters . is_empty () { filtered . retain (| test | opts . filters . iter () . any (| filter | matches_filter (test , filter))) ; } if ! opts . skip . is_empty () { filtered . retain (| test | ! opts . skip . iter () . any (| sf | matches_filter (test , sf))) ; } if opts . exclude_should_panic { filtered . retain (| test | test . desc . should_panic == ShouldPanic :: No) ; } match opts . run_ignored { RunIgnored :: Yes => { filtered . iter_mut () . for_each (| test | test . desc . ignore = false) ; } RunIgnored :: Only => { filtered . retain (| test | test . desc . ignore) ; filtered . iter_mut () . for_each (| test | test . desc . ignore = false) ; } RunIgnored :: No => { } } filtered }
};
}
