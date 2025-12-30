// Generated macro for UnifiedDiff (struct)
macro_rules! Depcrate_udiffUnifiedDiff {
() => {
// Module: crate::udiff
// Provides: {"UnifiedDiff"}
// Dependencies: {}
# [doc = " Unified diff formatter."] # [doc = ""] # [doc = " ```rust"] # [doc = " use similar::TextDiff;"] # [doc = " # let old_text = \"\";"] # [doc = " # let new_text = \"\";"] # [doc = " let text_diff = TextDiff::from_lines(old_text, new_text);"] # [doc = " print!(\"{}\", text_diff"] # [doc = "     .unified_diff()"] # [doc = "     .context_radius(10)"] # [doc = "     .header(\"old_file\", \"new_file\"));"] # [doc = " ```"] # [doc = ""] # [doc = " ## Unicode vs Bytes"] # [doc = ""] # [doc = " The [`UnifiedDiff`] type supports both unicode and byte diffs for all"] # [doc = " types compatible with [`DiffableStr`].  You can pick between the two"] # [doc = " versions by using [`UnifiedDiff.to_string`] or [`UnifiedDiff.to_writer`]."] # [doc = " The former uses [`DiffableStr::to_string_lossy`], the latter uses"] # [doc = " [`DiffableStr::as_bytes`] for each line."] pub struct UnifiedDiff < 'diff , 'old , 'new , 'bufs , T : DiffableStr + ? Sized > { diff : & 'diff TextDiff < 'old , 'new , 'bufs , T > , context_radius : usize , missing_newline_hint : bool , header : Option < (String , String) > , }
};
}
