// Generated macro for filter_normal_code (function)
macro_rules! Depcrate_commentfilter_normal_code {
() => {
// Module: crate::comment
// Provides: {"filter_normal_code"}
// Dependencies: {}
pub (crate) fn filter_normal_code (code : & str) -> String { let mut buffer = String :: with_capacity (code . len ()) ; LineClasses :: new (code) . for_each (| (kind , line) | match kind { FullCodeCharKind :: Normal | FullCodeCharKind :: StartString | FullCodeCharKind :: InString | FullCodeCharKind :: EndString => { buffer . push_str (& line) ; buffer . push ('\n') ; } _ => () , }) ; if ! code . ends_with ('\n') && buffer . ends_with ('\n') { buffer . pop () ; } buffer }
};
}
