// Generated macro for pub_if_not_doc (macro)
macro_rules! Depcrate_macrospub_if_not_doc {
() => {
// Module: crate::macros
// Provides: {"pub_if_not_doc"}
// Dependencies: {}
# [cfg (all (not (doc) , feature = "parsing"))] macro_rules ! pub_if_not_doc { ($ (# [$ m : meta]) * $ pub : ident $ ($ item : tt) *) => { check_keyword_matches ! (pub $ pub) ; $ (# [$ m]) * $ pub $ ($ item) * } ; }
};
}
