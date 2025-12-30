// Generated macro for test_hash_no_concat_alias (function)
macro_rules! Depcrate_teststest_hash_no_concat_alias {
() => {
// Module: crate::tests
// Provides: {"test_hash_no_concat_alias"}
// Dependencies: {}
# [test] fn test_hash_no_concat_alias () { let s = ("aa" , "bb") ; let t = ("aabb" , "") ; let u = ("a" , "abb") ; assert ! (s != t && t != u) ; assert ! (hash (& s) != hash (& t) && hash (& s) != hash (& u)) ; let u = [1 , 0 , 0 , 0] ; let v = (& u [.. 1] , & u [1 .. 3] , & u [3 ..]) ; let w = (& u [..] , & u [4 .. 4] , & u [4 .. 4]) ; assert_ne ! (v , w) ; assert_ne ! (hash (& v) , hash (& w)) ; }
};
}
