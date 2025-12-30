// Generated macro for windows (module)
macro_rules! Depcratewindows {
() => {
// Module: crate
// Provides: {"windows"}
// Dependencies: {}
# [doc = " Windows-specific escaping."] pub mod windows { use std :: borrow :: Cow ; use std :: iter :: repeat ; # [doc = " Escape for the windows cmd.exe shell."] # [doc = ""] # [doc = " See [here][msdn] for more information."] # [doc = ""] # [doc = " [msdn]: http://blogs.msdn.com/b/twistylittlepassagesallalike/archive/2011/04/23/everyone-quotes-arguments-the-wrong-way.aspx"] pub fn escape (s : Cow < str >) -> Cow < str > { let mut needs_escape = s . is_empty () ; for ch in s . chars () { match ch { '"' | '\t' | '\n' | ' ' => needs_escape = true , _ => { } } } if ! needs_escape { return s } let mut es = String :: with_capacity (s . len ()) ; es . push ('"') ; let mut chars = s . chars () . peekable () ; loop { let mut nslashes = 0 ; while let Some (& '\\') = chars . peek () { chars . next () ; nslashes += 1 ; } match chars . next () { Some ('"') => { es . extend (repeat ('\\') . take (nslashes * 2 + 1)) ; es . push ('"') ; } Some (c) => { es . extend (repeat ('\\') . take (nslashes)) ; es . push (c) ; } None => { es . extend (repeat ('\\') . take (nslashes * 2)) ; break ; } } } es . push ('"') ; es . into () } # [test] fn test_escape () { assert_eq ! (escape ("--aaa=bbb-ccc" . into ()) , "--aaa=bbb-ccc") ; assert_eq ! (escape ("linker=gcc -L/foo -Wl,bar" . into ()) , r#""linker=gcc -L/foo -Wl,bar""#) ; assert_eq ! (escape (r#"--features="default""# . into ()) , r#""--features=\"default\"""#) ; assert_eq ! (escape (r#"\path\to\my documents\"# . into ()) , r#""\path\to\my documents\\""#) ; assert_eq ! (escape ("" . into ()) , r#""""#) ; } }
};
}
