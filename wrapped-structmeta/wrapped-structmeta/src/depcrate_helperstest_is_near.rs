// Generated macro for test_is_near (function)
macro_rules! Depcrate_helperstest_is_near {
() => {
// Module: crate::helpers
// Provides: {"test_is_near"}
// Dependencies: {}
# [test] fn test_is_near () { fn check (s0 : & str , s1 : & str , e : Option < usize >) { let c0 : Vec < _ > = s0 . chars () . collect () ; let c1 : Vec < _ > = s1 . chars () . collect () ; assert_eq ! (distance (& c0 , & c1) , e , "{s0} , {s1}") } check ("a" , "a" , Some (0)) ; check ("a" , "b" , Some (1)) ; check ("a" , "ab" , Some (1)) ; check ("ab" , "a" , Some (1)) ; check ("a" , "aa" , Some (1)) ; check ("ab" , "ba" , Some (2)) ; }
};
}
