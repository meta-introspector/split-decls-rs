// Generated macro for filter_inline_attrs (function)
macro_rules! Depcrate_attrfilter_inline_attrs {
() => {
// Module: crate::attr
// Provides: {"filter_inline_attrs"}
// Dependencies: {}
# [doc = " Returns attributes that are within `outer_span`."] pub (crate) fn filter_inline_attrs (attrs : & [ast :: Attribute] , outer_span : Span) -> ast :: AttrVec { attrs . iter () . filter (| a | outer_span . lo () <= a . span . lo () && a . span . hi () <= outer_span . hi ()) . cloned () . collect () }
};
}
