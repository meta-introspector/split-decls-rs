// Generated macro for parse_cfgs (function)
macro_rules! Depcrate_palparse_cfgs {
() => {
// Module: crate::pal
// Provides: {"parse_cfgs"}
// Dependencies: {}
fn parse_cfgs (contents : & str) -> Vec < (usize , & str) > { let candidate_cfgs = contents . match_indices ("cfg") ; let candidate_cfg_idxs = candidate_cfgs . map (| (i , _) | i) ; let cfgs = candidate_cfg_idxs . filter (| i | { let pre_idx = i . saturating_sub (1) ; let succeeds_non_ident = ! contents . as_bytes () . get (pre_idx) . cloned () . map (char :: from) . map (char :: is_alphanumeric) . unwrap_or (false) ; let contents_after = & contents [* i ..] ; let first_paren = contents_after . find ('(') ; let paren_idx = first_paren . map (| ip | i + ip) ; let preceeds_whitespace_and_paren = paren_idx . map (| ip | { let maybe_space = & contents [* i + "cfg" . len () .. ip] ; maybe_space . chars () . all (| c | char :: is_whitespace (c) || c == '!') }) . unwrap_or (false) ; succeeds_non_ident && preceeds_whitespace_and_paren }) ; cfgs . flat_map (| i | { let mut depth = 0 ; let contents_from = & contents [i ..] ; for (j , byte) in contents_from . bytes () . enumerate () { match byte { b'(' => { depth += 1 ; } b')' => { depth -= 1 ; if depth == 0 { return Some ((i , & contents_from [..= j])) ; } } _ => { } } } None }) . collect () }
};
}
