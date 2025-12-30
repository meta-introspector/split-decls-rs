// Generated macro for blo_test (function)
macro_rules! Depcrate_percentblo_test {
() => {
// Module: crate::percent
// Provides: {"blo_test"}
// Dependencies: {}
# [test] fn blo_test () { use writeable :: assert_writeable_eq ; let blo_positive_pattern = create_unsigned_pattern ("% #,#0" , "%") . unwrap () ; assert_writeable_eq ! (blo_positive_pattern . interpolate (["123"]) , "% 123") ; let blo_negative_pattern = create_signed_pattern ("% -#,#0" , "%") . unwrap () ; assert_writeable_eq ! (blo_negative_pattern . interpolate (["123" , "+"]) , "% +123") ; assert_writeable_eq ! (blo_negative_pattern . interpolate (["123" , "-"]) , "% -123") ; assert_writeable_eq ! (blo_negative_pattern . interpolate (["123" , "~"]) , "% ~123") ; }
};
}
