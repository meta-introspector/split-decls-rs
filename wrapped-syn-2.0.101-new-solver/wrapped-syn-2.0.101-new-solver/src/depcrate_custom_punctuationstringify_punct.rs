// Generated macro for stringify_punct (macro)
macro_rules! Depcrate_custom_punctuationstringify_punct {
() => {
// Module: crate::custom_punctuation
// Provides: {"stringify_punct"}
// Dependencies: {}
# [doc (hidden)] # [macro_export] macro_rules ! stringify_punct { ($ ($ tt : tt) +) => { $ crate :: __private :: concat ! ($ ($ crate :: __private :: stringify ! ($ tt)) ,+) } ; }
};
}
