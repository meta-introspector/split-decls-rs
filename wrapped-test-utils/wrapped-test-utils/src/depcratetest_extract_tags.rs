// Generated macro for test_extract_tags (function)
macro_rules! Depcratetest_extract_tags {
() => {
// Module: crate
// Provides: {"test_extract_tags"}
// Dependencies: {}
# [test] fn test_extract_tags () { let (tags , text) = extract_tags (r#"<tag fn>fn <tag>main</tag>() {}</tag>"# , "tag") ; let actual = tags . into_iter () . map (| (range , attr) | (& text [range] , attr)) . collect :: < Vec < _ > > () ; assert_eq ! (actual , vec ! [("fn main() {}" , Some ("fn" . into ())) , ("main" , None) ,]) ; }
};
}
