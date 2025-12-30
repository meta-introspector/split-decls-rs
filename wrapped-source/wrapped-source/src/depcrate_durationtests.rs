// Generated macro for tests (module)
macro_rules! Depcrate_durationtests {
() => {
// Module: crate::duration
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn test_strip_padded_character () { let mut s = "hhmmss" ; assert_eq ! (strip_padded_character (& mut s , 'h') , 2) ; assert_eq ! (strip_padded_character (& mut s , 'm') , 2) ; assert_eq ! (strip_padded_character (& mut s , 's') , 2) ; assert_eq ! (s , "") ; } # [test] fn test_strip_separated_padded_characters () { let mut s = "hh:mm:ss" ; let (padding , sep) = strip_separated_padded_characters (& mut s , ['h' , 'm' , 's']) . unwrap () ; assert_eq ! (padding , [2 , 2 , 2]) ; assert_eq ! (sep , ":") ; assert_eq ! (s , "") ; let mut s = "h:mm:ss" ; let (padding , sep) = strip_separated_padded_characters (& mut s , ['h' , 'm' , 's']) . unwrap () ; assert_eq ! (padding , [1 , 2 , 2]) ; assert_eq ! (sep , ":") ; assert_eq ! (s , "") ; } }
};
}
