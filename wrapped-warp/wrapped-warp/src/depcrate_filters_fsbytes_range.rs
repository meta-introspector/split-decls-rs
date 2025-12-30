// Generated macro for bytes_range (function)
macro_rules! Depcrate_filters_fsbytes_range {
() => {
// Module: crate::filters::fs
// Provides: {"bytes_range"}
// Dependencies: {}
fn bytes_range (range : Option < Range > , max_len : u64) -> Result < (u64 , u64) , BadRange > { use std :: ops :: Bound ; let range = if let Some (range) = range { range } else { return Ok ((0 , max_len)) ; } ; let ret = range . satisfiable_ranges (max_len) . map (| (start , end) | { let start = match start { Bound :: Unbounded => 0 , Bound :: Included (s) => s , Bound :: Excluded (s) => s + 1 , } ; let end = match end { Bound :: Unbounded => max_len , Bound :: Included (s) => { if s == max_len { s } else { s + 1 } } Bound :: Excluded (s) => s , } ; if start < end && end <= max_len { Ok ((start , end)) } else { tracing :: trace ! ("unsatisfiable byte range: {}-{}/{}" , start , end , max_len) ; Err (BadRange) } }) . next () . unwrap_or (Ok ((0 , max_len))) ; ret }
};
}
