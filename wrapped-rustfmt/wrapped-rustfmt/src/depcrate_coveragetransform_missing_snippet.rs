// Generated macro for transform_missing_snippet (function)
macro_rules! Depcrate_coveragetransform_missing_snippet {
() => {
// Module: crate::coverage
// Provides: {"transform_missing_snippet"}
// Dependencies: {}
pub (crate) fn transform_missing_snippet < 'a > (config : & Config , string : & 'a str) -> Cow < 'a , str > { match config . emit_mode () { EmitMode :: Coverage => Cow :: from (replace_chars (string)) , _ => Cow :: from (string) , } }
};
}
