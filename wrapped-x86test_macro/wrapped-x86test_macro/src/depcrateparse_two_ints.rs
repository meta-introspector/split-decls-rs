// Generated macro for parse_two_ints (function)
macro_rules! Depcrateparse_two_ints {
() => {
// Module: crate
// Provides: {"parse_two_ints"}
// Dependencies: {}
# [doc = " Parse two integer literals from args (e.g., like (1, 2))."] fn parse_two_ints (args : Punctuated < NestedMeta , Comma >) -> (u64 , u64) { if args . len () != 2 { args . span () . unstable () . error ("needs two numbers as parameters") . emit () ; } let a = if let NestedMeta :: Literal (Lit :: Int (first)) = & args [0] { first . value () } else { args [0] . span () . unstable () . error ("first parameter not an int literal") . emit () ; 0 } ; let b = if let NestedMeta :: Literal (Lit :: Int (second)) = & args [1] { second . value () } else { args [1] . span () . unstable () . error ("second parameter not an int literal") . emit () ; 0 } ; (a , b) }
};
}
