// Generated macro for get_editions_from_comments (function)
macro_rules! Depcrate_testget_editions_from_comments {
() => {
// Module: crate::test
// Provides: {"get_editions_from_comments"}
// Dependencies: {}
fn get_editions_from_comments (comments : & HashMap < String , String > ,) -> (Option < Edition > , Option < StyleEdition > , Option < Version >) { (comments . get ("edition") . map (| e | Edition :: from_str (e) . expect (& format ! ("invalid edition value: '{}'" , e))) , comments . get ("style_edition") . map (| se | { StyleEdition :: from_str (se) . expect (& format ! ("invalid style_edition value: '{}'" , se)) }) , comments . get ("version") . map (| v | Version :: from_str (v) . expect (& format ! ("invalid version value: '{}'" , v))) ,) }
};
}
