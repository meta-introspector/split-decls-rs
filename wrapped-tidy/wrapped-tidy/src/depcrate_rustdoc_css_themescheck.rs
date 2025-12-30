// Generated macro for check (function)
macro_rules! Depcrate_rustdoc_css_themescheck {
() => {
// Module: crate::rustdoc_css_themes
// Provides: {"check"}
// Dependencies: {}
pub fn check (librustdoc_path : & Path , bad : & mut bool) { let rustdoc_css = "html/static/css/rustdoc.css" ; let noscript_css = "html/static/css/noscript.css" ; let rustdoc_css_contents = std :: fs :: read_to_string (librustdoc_path . join (rustdoc_css)) . unwrap_or_else (| e | panic ! ("failed to read librustdoc/{rustdoc_css}: {e}")) ; let noscript_css_contents = std :: fs :: read_to_string (librustdoc_path . join (noscript_css)) . unwrap_or_else (| e | panic ! ("failed to read librustdoc/{noscript_css}: {e}")) ; compare_themes_from_files ("light" , rustdoc_css_contents . lines () . enumerate () . map (| (i , l) | (i + 1 , l . trim ())) , noscript_css_contents . lines () . enumerate () . map (| (i , l) | (i + 1 , l . trim ())) , bad ,) ; compare_themes_from_files ("dark" , rustdoc_css_contents . lines () . enumerate () , noscript_css_contents . lines () . enumerate () , bad ,) ; }
};
}
