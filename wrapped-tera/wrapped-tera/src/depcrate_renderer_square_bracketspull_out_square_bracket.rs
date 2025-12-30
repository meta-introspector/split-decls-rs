// Generated macro for pull_out_square_bracket (function)
macro_rules! Depcrate_renderer_square_bracketspull_out_square_bracket {
() => {
// Module: crate::renderer::square_brackets
// Provides: {"pull_out_square_bracket"}
// Dependencies: {}
# [doc = " Return a Vec of all substrings contained in '[ ]'s"] # [doc = " Ignore quoted strings and integers."] pub fn pull_out_square_bracket (s : & str) -> Vec < String > { let mut chars = s . chars () ; let mut results = vec ! [] ; loop { match chars . next () { Some ('[') => { let c = chars . next () . unwrap () ; if c != '"' && c != '\'' { let mut inside_bracket = vec ! [c] ; let mut bracket_count = 1 ; loop { let c = chars . next () ; match c { Some (']') => bracket_count -= 1 , Some ('[') => bracket_count += 1 , Some (_) => () , None => break , } ; if bracket_count == 0 { let sub : String = inside_bracket . into_iter () . collect () ; if sub . parse :: < usize > () . is_err () { results . push (sub) ; } break ; } inside_bracket . push (c . unwrap ()) ; } } } None => break , _ => () , } } results }
};
}
