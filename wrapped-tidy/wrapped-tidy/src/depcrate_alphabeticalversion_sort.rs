// Generated macro for version_sort (function)
macro_rules! Depcrate_alphabeticalversion_sort {
() => {
// Module: crate::alphabetical
// Provides: {"version_sort"}
// Dependencies: {}
fn version_sort (a : & str , b : & str) -> Ordering { let mut it1 = a . chars () . peekable () ; let mut it2 = b . chars () . peekable () ; while let (Some (x) , Some (y)) = (it1 . peek () , it2 . peek ()) { match (x . is_numeric () , y . is_numeric ()) { (true , true) => { let num1 : String = consume_numeric_prefix (it1 . by_ref ()) ; let num2 : String = consume_numeric_prefix (it2 . by_ref ()) ; let int1 : u64 = num1 . parse () . unwrap () ; let int2 : u64 = num2 . parse () . unwrap () ; match int1 . cmp (& int2) . then_with (| | num1 . cmp (& num2)) { Ordering :: Equal => continue , different => return different , } } (false , false) => match x . cmp (y) { Ordering :: Equal => { it1 . next () ; it2 . next () ; continue ; } different => return different , } , (false , true) | (true , false) => { return x . cmp (y) ; } } } it1 . next () . cmp (& it2 . next ()) }
};
}
