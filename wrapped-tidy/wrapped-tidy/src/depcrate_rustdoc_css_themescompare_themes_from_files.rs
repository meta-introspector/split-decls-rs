// Generated macro for compare_themes_from_files (function)
macro_rules! Depcrate_rustdoc_css_themescompare_themes_from_files {
() => {
// Module: crate::rustdoc_css_themes
// Provides: {"compare_themes_from_files"}
// Dependencies: {}
fn compare_themes_from_files < 'a > (name : & str , mut rustdoc_css_lines : impl Iterator < Item = (usize , & 'a str) > , mut noscript_css_lines : impl Iterator < Item = (usize , & 'a str) > , bad : & mut bool ,) { let begin_theme_pat = format ! ("/* Begin theme: {name}") ; let mut found_theme = None ; let mut found_theme_noscript = None ; while let Some ((rustdoc_css_line_number , rustdoc_css_line)) = rustdoc_css_lines . next () { if ! rustdoc_css_line . starts_with (& begin_theme_pat) { continue ; } if let Some (found_theme) = found_theme { tidy_error ! (bad , "rustdoc.css contains two {name} themes on lines {rustdoc_css_line_number} and {found_theme}" ,) ; return ; } found_theme = Some (rustdoc_css_line_number) ; while let Some ((noscript_css_line_number , noscript_css_line)) = noscript_css_lines . next () { if ! noscript_css_line . starts_with (& begin_theme_pat) { continue ; } if let Some (found_theme_noscript) = found_theme_noscript { tidy_error ! (bad , "noscript.css contains two {name} themes on lines {noscript_css_line_number} and {found_theme_noscript}" ,) ; return ; } found_theme_noscript = Some (noscript_css_line_number) ; compare_themes (name , & mut rustdoc_css_lines , & mut noscript_css_lines , bad) ; } } }
};
}
