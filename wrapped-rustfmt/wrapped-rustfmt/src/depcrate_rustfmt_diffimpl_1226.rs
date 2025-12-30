// Generated macro for impl_1226 (impl)
macro_rules! Depcrate_rustfmt_diffimpl_1226 {
() => {
// Module: crate::rustfmt_diff
// Provides: {"impl_1226"}
// Dependencies: {}
impl std :: str :: FromStr for ModifiedLines { type Err = () ; fn from_str (s : & str) -> Result < ModifiedLines , () > { let mut chunks = vec ! [] ; let mut lines = s . lines () ; while let Some (header) = lines . next () { let mut header = header . split_whitespace () ; let (orig , rem , new_lines) = match (header . next () , header . next () , header . next ()) { (Some (orig) , Some (removed) , Some (added)) => (orig , removed , added) , _ => return Err (()) , } ; let (orig , rem , new_lines) : (u32 , u32 , usize) = match (orig . parse () , rem . parse () , new_lines . parse ()) { (Ok (a) , Ok (b) , Ok (c)) => (a , b , c) , _ => return Err (()) , } ; let lines = lines . by_ref () . take (new_lines) ; let lines : Vec < _ > = lines . map (ToOwned :: to_owned) . collect () ; if lines . len () != new_lines { return Err (()) ; } chunks . push (ModifiedChunk { line_number_orig : orig , lines_removed : rem , lines , }) ; } Ok (ModifiedLines { chunks }) } }
};
}
