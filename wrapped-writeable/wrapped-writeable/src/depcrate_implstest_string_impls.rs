// Generated macro for test_string_impls (function)
macro_rules! Depcrate_implstest_string_impls {
() => {
// Module: crate::impls
// Provides: {"test_string_impls"}
// Dependencies: {}
# [test] fn test_string_impls () { fn check_writeable_slice < W : Writeable + core :: fmt :: Display > (writeables : & [W]) { assert_writeable_eq ! (& writeables [0] , "") ; assert_writeable_eq ! (& writeables [1] , "abc") ; assert ! (matches ! (writeables [0] . write_to_string () , Cow :: Borrowed (_))) ; assert ! (matches ! (writeables [1] . write_to_string () , Cow :: Borrowed (_))) ; } let arr : & [& str] = & ["" , "abc"] ; check_writeable_slice (arr) ; let arr : & [String] = & [String :: new () , "abc" . to_owned ()] ; check_writeable_slice (arr) ; let chars = ['a' , 'β' , '你' , '😀'] ; for i in 0 .. chars . len () { let s = String :: from (chars [i]) ; assert_writeable_eq ! (& chars [i] , s) ; for j in 0 .. chars . len () { assert_eq ! (crate :: cmp_str (& chars [j] , & s) , chars [j] . cmp (& chars [i]) , "{:?} vs {:?}" , chars [j] , chars [i]) ; } } let arr : & [Cow < str >] = & [Cow :: Borrowed ("") , Cow :: Owned ("abc" . to_string ())] ; check_writeable_slice (arr) ; let arr : & [Box < str >] = & ["" . into () , "abc" . into ()] ; check_writeable_slice (arr) ; let arr : & [alloc :: rc :: Rc < str >] = & ["" . into () , "abc" . into ()] ; check_writeable_slice (arr) ; let arr : & [alloc :: sync :: Arc < str >] = & ["" . into () , "abc" . into ()] ; check_writeable_slice (arr) ; let arr : & [& String] = & [& String :: new () , & "abc" . to_owned ()] ; check_writeable_slice (arr) ; }
};
}
