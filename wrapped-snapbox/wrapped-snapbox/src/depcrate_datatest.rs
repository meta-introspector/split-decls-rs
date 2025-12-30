// Generated macro for test (module)
macro_rules! Depcrate_datatest {
() => {
// Module: crate::data
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: * ; # [track_caller] fn validate_cases (cases : & [(& str , bool)] , input_format : DataFormat) { for (input , valid) in cases . iter () . copied () { let (expected_is_format , expected_coerced_format) = if valid { (input_format , input_format) } else { (DataFormat :: Error , DataFormat :: Text) } ; let actual_is = Data :: text (input) . is (input_format) ; assert_eq ! (actual_is . format () , expected_is_format , "\n{input}\n{actual_is}") ; let actual_coerced = Data :: text (input) . coerce_to (input_format) ; assert_eq ! (actual_coerced . format () , expected_coerced_format , "\n{input}\n{actual_coerced}") ; if valid { assert_eq ! (actual_is , actual_coerced) ; let rendered = actual_is . render () . unwrap () ; let bytes = actual_is . to_bytes () . unwrap () ; assert_eq ! (rendered , std :: str :: from_utf8 (& bytes) . unwrap ()) ; assert_eq ! (Data :: text (& rendered) . is (input_format) , actual_is) ; } } } # [test] fn text () { let cases = [("" , true) , ("good" , true) , ("{}" , true) , ("\"\"" , true)] ; validate_cases (& cases , DataFormat :: Text) ; } # [cfg (feature = "json")] # [test] fn json () { let cases = [("" , false) , ("bad" , false) , ("{}" , true) , ("\"\"" , true)] ; validate_cases (& cases , DataFormat :: Json) ; } # [cfg (feature = "json")] # [test] fn jsonlines () { let cases = [("" , true) , ("bad" , false) , ("{}" , true) , ("\"\"" , true) , ("
{}
{}
" , true ,) , ("
{}

{}
" , true ,) , ("
{}
bad
{}
" , false ,) ,] ; validate_cases (& cases , DataFormat :: JsonLines) ; } }
};
}
