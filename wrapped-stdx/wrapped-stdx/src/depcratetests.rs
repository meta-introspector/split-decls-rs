// Generated macro for tests (module)
macro_rules! Depcratetests {
() => {
// Module: crate
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn test_trim_indent () { assert_eq ! (trim_indent ("") , "") ; assert_eq ! (trim_indent ("
            hello
            world
") , "hello\nworld\n") ; assert_eq ! (trim_indent ("
            hello
            world") , "hello\nworld") ; assert_eq ! (trim_indent ("    hello\n    world\n") , "hello\nworld\n") ; assert_eq ! (trim_indent ("
            fn main() {
                return 92;
            }
        ") , "fn main() {\n    return 92;\n}\n") ; } # [test] fn test_replace () { # [track_caller] fn test_replace (src : & str , from : char , to : & str , expected : & str) { let mut s = src . to_owned () ; replace (& mut s , from , to) ; assert_eq ! (s , expected , "from: {from:?}, to: {to:?}") ; } test_replace ("" , 'a' , "b" , "") ; test_replace ("" , 'a' , "😀" , "") ; test_replace ("" , '😀' , "a" , "") ; test_replace ("a" , 'a' , "b" , "b") ; test_replace ("aa" , 'a' , "b" , "bb") ; test_replace ("ada" , 'a' , "b" , "bdb") ; test_replace ("a" , 'a' , "😀" , "😀") ; test_replace ("😀" , '😀' , "a" , "a") ; test_replace ("😀x" , '😀' , "a" , "ax") ; test_replace ("y😀x" , '😀' , "a" , "yax") ; test_replace ("a,b,c" , ',' , "." , "a.b.c") ; test_replace ("a,b,c" , ',' , ".." , "a..b..c") ; test_replace ("a.b.c" , '.' , ".." , "a..b..c") ; test_replace ("a.b.c" , '.' , ".." , "a..b..c") ; test_replace ("a😀b😀c" , '😀' , "." , "a.b.c") ; test_replace ("a.b.c" , '.' , "😀" , "a😀b😀c") ; test_replace ("a.b.c" , '.' , "😀😀" , "a😀😀b😀😀c") ; test_replace (".a.b.c." , '.' , "()" , "()a()b()c()") ; test_replace (".a.b.c." , '.' , "" , "abc") ; } }
};
}
