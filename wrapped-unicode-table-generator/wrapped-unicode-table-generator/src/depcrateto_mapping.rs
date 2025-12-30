// Generated macro for to_mapping (function)
macro_rules! Depcrateto_mapping {
() => {
// Module: crate
// Provides: {"to_mapping"}
// Dependencies: {}
fn to_mapping (origin : u32 , codepoints : Vec < ucd_parse :: Codepoint >) -> Option < [u32 ; 3] > { let mut a = None ; let mut b = None ; let mut c = None ; for codepoint in codepoints { if origin == codepoint . value () { return None ; } if a . is_none () { a = Some (codepoint . value ()) ; } else if b . is_none () { b = Some (codepoint . value ()) ; } else if c . is_none () { c = Some (codepoint . value ()) ; } else { panic ! ("more than 3 mapped codepoints") } } Some ([a . unwrap () , b . unwrap_or (0) , c . unwrap_or (0)]) }
};
}
