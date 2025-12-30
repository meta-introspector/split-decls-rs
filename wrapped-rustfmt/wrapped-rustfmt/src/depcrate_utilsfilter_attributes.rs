// Generated macro for filter_attributes (function)
macro_rules! Depcrate_utilsfilter_attributes {
() => {
// Module: crate::utils
// Provides: {"filter_attributes"}
// Dependencies: {}
# [inline] pub (crate) fn filter_attributes (attrs : & [ast :: Attribute] , style : ast :: AttrStyle ,) -> Vec < ast :: Attribute > { attrs . iter () . filter (| a | a . style == style) . cloned () . collect :: < Vec < _ > > () }
};
}
