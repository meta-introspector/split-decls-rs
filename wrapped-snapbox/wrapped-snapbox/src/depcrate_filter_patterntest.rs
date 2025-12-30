// Generated macro for test (module)
macro_rules! Depcrate_filter_patterntest {
() => {
// Module: crate::filter::pattern
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: * ; # [test] fn str_normalize_redactions_line_matches_cases () { let cases = [("" , "" , true) , ("" , "[..]" , true) , ("hello" , "hello" , true) , ("hello" , "goodbye" , false) , ("hello" , "[..]" , true) , ("hello" , "he[..]" , true) , ("hello" , "go[..]" , false) , ("hello" , "[..]o" , true) , ("hello" , "[..]e" , false) , ("hello" , "he[..]o" , true) , ("hello" , "he[..]e" , false) , ("hello" , "go[..]o" , false) , ("hello" , "go[..]e" , false) , ("hello world, goodbye moon" , "hello [..], goodbye [..]" , true ,) , ("hello world, goodbye moon" , "goodbye [..], goodbye [..]" , false ,) , ("hello world, goodbye moon" , "goodbye [..], hello [..]" , false ,) , ("hello world, goodbye moon" , "hello [..], [..] moon" , true) , ("hello world, goodbye moon" , "goodbye [..], [..] moon" , false ,) , ("hello world, goodbye moon" , "hello [..], [..] world" , false) ,] ; for (line , pattern , expected) in cases { let actual = line_matches (line , pattern , & Redactions :: new ()) ; assert_eq ! (expected , actual , "line={line:?}  pattern={pattern:?}") ; } } }
};
}
