// Generated macro for missing_span_before_after_where (function)
macro_rules! Depcrate_itemsmissing_span_before_after_where {
() => {
// Module: crate::items
// Provides: {"missing_span_before_after_where"}
// Dependencies: {}
fn missing_span_before_after_where (before_item_span_end : BytePos , predicates : & [ast :: WherePredicate] , where_span : Span ,) -> (Span , Span) { let missing_span_before = mk_sp (before_item_span_end , where_span . lo ()) ; let pos_after_where = where_span . lo () + BytePos (5) ; let missing_span_after = mk_sp (pos_after_where , predicates [0] . span () . lo ()) ; (missing_span_before , missing_span_after) }
};
}
