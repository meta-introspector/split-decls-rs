// Generated macro for for_each_error_annotation_revision (function)
macro_rules! Depcrate_unknown_revisionfor_each_error_annotation_revision {
() => {
// Module: crate::unknown_revision
// Provides: {"for_each_error_annotation_revision"}
// Dependencies: {}
fn for_each_error_annotation_revision < 'a > (contents : & 'a str , callback : & mut dyn FnMut (ErrorAnnRev < 'a >) ,) { let error_regex = { static RE : OnceLock < Regex > = OnceLock :: new () ; RE . get_or_init (| | Regex :: new (r"//\[(?<revs>[^]]*)\]~") . unwrap ()) } ; for (line_number , line) in (1 ..) . zip (contents . lines ()) { let Some (captures) = error_regex . captures (line) else { continue } ; for revision in captures . name ("revs") . unwrap () . as_str () . split (',') { callback (ErrorAnnRev { line_number , revision }) ; } } }
};
}
