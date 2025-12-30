// Generated macro for test_get_close_matches (function)
macro_rules! Depcrate_texttest_get_close_matches {
() => {
// Module: crate::text
// Provides: {"test_get_close_matches"}
// Dependencies: {}
# [test] fn test_get_close_matches () { let matches = get_close_matches ("appel" , & ["ape" , "apple" , "peach" , "puppy"] [..] , 3 , 0.6) ; assert_eq ! (matches , vec ! ["apple" , "ape"]) ; let matches = get_close_matches ("hulo" , & ["hi" , "hulu" , "hali" , "hoho" , "amaz" , "zulo" , "blah" , "hopp" , "uulo" , "aulo" ,] [..] , 5 , 0.7 ,) ; assert_eq ! (matches , vec ! ["aulo" , "hulu" , "uulo" , "zulo"]) ; }
};
}
