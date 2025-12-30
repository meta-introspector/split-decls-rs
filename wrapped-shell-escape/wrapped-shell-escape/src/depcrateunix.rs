// Generated macro for unix (module)
macro_rules! Depcrateunix {
() => {
// Module: crate
// Provides: {"unix"}
// Dependencies: {}
# [doc = " Unix-specific escaping."] pub mod unix { use std :: borrow :: Cow ; fn non_whitelisted (ch : char) -> bool { match ch { 'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '-' | '_' | '=' | '/' | ',' | '.' | '+' => false , _ => true , } } # [doc = " Escape characters that may have special meaning in a shell, including spaces."] pub fn escape (s : Cow < str >) -> Cow < str > { if ! s . is_empty () && ! s . contains (non_whitelisted) { return s ; } let mut es = String :: with_capacity (s . len () + 2) ; es . push ('\'') ; for ch in s . chars () { match ch { '\'' | '!' => { es . push_str ("'\\") ; es . push (ch) ; es . push ('\'') ; } _ => es . push (ch) , } } es . push ('\'') ; es . into () } # [test] fn test_escape () { assert_eq ! (escape ("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789-_=/,.+" . into ()) , "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789-_=/,.+") ; assert_eq ! (escape ("--aaa=bbb-ccc" . into ()) , "--aaa=bbb-ccc") ; assert_eq ! (escape ("linker=gcc -L/foo -Wl,bar" . into ()) , r#"'linker=gcc -L/foo -Wl,bar'"#) ; assert_eq ! (escape (r#"--features="default""# . into ()) , r#"'--features="default"'"#) ; assert_eq ! (escape (r#"'!\$`\\\n "# . into ()) , r#"''\'''\!'\$`\\\n '"#) ; assert_eq ! (escape ("" . into ()) , r#"''"#) ; } }
};
}
