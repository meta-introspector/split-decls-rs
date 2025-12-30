// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () { let names = ["Joe" , "Bob" , "Alice"] ; let compact = names . join_compact (", ") ; assert_eq ! (compact , "Joe, Bob, Alice") ; println ! ("{}" , compact) ; let fruits = ["apple" , "orange" , "banana"] ; let compact = fruits . concat_compact () ; assert_eq ! (compact , "appleorangebanana") ; println ! ("{}" , compact) ; let number = 42 ; let compact = number . to_compact_string () ; assert_eq ! (compact , "42") ; println ! ("{}" , compact) ; let answer = true ; let compact = answer . to_compact_string () ; assert_eq ! (compact , "true") ; println ! ("{}" , compact) ; }
};
}
