// Generated macro for normalize_ranges (function)
macro_rules! Depcrate_config_file_linesnormalize_ranges {
() => {
// Module: crate::config::file_lines
// Provides: {"normalize_ranges"}
// Dependencies: {}
# [doc = " Normalizes the ranges so that the invariants for `FileLines` hold: ranges are non-overlapping,"] # [doc = " and ordered by their start point."] fn normalize_ranges (ranges : & mut HashMap < FileName , Vec < Range > >) { for ranges in ranges . values_mut () { ranges . sort () ; let mut result = vec ! [] ; let mut iter = ranges . iter_mut () . peekable () ; while let Some (next) = iter . next () { let mut next = * next ; while let Some (& & mut peek) = iter . peek () { if let Some (merged) = next . merge (peek) { iter . next () . unwrap () ; next = merged ; } else { break ; } } result . push (next) } * ranges = result ; } }
};
}
