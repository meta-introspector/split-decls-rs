// Generated macro for impl_1224 (impl)
macro_rules! Depcrate_rustfmt_diffimpl_1224 {
() => {
// Module: crate::rustfmt_diff
// Provides: {"impl_1224"}
// Dependencies: {}
impl From < Vec < Mismatch > > for ModifiedLines { fn from (mismatches : Vec < Mismatch >) -> ModifiedLines { let chunks = mismatches . into_iter () . map (| mismatch | { let lines = mismatch . lines . iter () ; let num_removed = lines . filter (| line | matches ! (line , DiffLine :: Resulting (_))) . count () ; let new_lines = mismatch . lines . into_iter () . filter_map (| line | match line { DiffLine :: Context (_) | DiffLine :: Resulting (_) => None , DiffLine :: Expected (str) => Some (str) , }) ; ModifiedChunk { line_number_orig : mismatch . line_number_orig , lines_removed : num_removed as u32 , lines : new_lines . collect () , } }) ; ModifiedLines { chunks : chunks . collect () , } } }
};
}
