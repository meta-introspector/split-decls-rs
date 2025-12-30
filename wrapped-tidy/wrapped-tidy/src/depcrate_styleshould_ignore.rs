// Generated macro for should_ignore (function)
macro_rules! Depcrate_styleshould_ignore {
() => {
// Module: crate::style
// Provides: {"should_ignore"}
// Dependencies: {}
# [doc = " Returns `true` if `line` can be ignored. This is the case when it contains"] # [doc = " an annotation that is explicitly ignored."] fn should_ignore (line : & str) -> bool { static_regex ! ("\\s*//(\\[.*\\])?~.*") . is_match (line) || ANNOTATIONS_TO_IGNORE . iter () . any (| a | line . contains (a)) || static_regex ! ("\\s*//@(\\[.*\\]) (compile-flags|normalize-stderr|error-pattern).*") . is_match (line) || static_regex ! ("\\s*//@ \\!?(count|files|has|has-dir|hasraw|matches|matchesraw|snapshot)\\s.*") . is_match (line) }
};
}
