// Generated macro for compare_themes (function)
macro_rules! Depcrate_rustdoc_css_themescompare_themes {
() => {
// Module: crate::rustdoc_css_themes
// Provides: {"compare_themes"}
// Dependencies: {}
fn compare_themes < 'a > (name : & str , rustdoc_css_lines : impl Iterator < Item = (usize , & 'a str) > , noscript_css_lines : impl Iterator < Item = (usize , & 'a str) > , bad : & mut bool ,) { let end_theme_pat = format ! ("/* End theme: {name}") ; for ((rustdoc_css_line_number , rustdoc_css_line) , (noscript_css_line_number , noscript_css_line) ,) in rustdoc_css_lines . zip (noscript_css_lines) { if noscript_css_line . starts_with (":root, :root:not([data-theme]) {") && (rustdoc_css_line . starts_with (& format ! (r#":root[data-theme="{name}"] {{"#)) || rustdoc_css_line . starts_with (& format ! (r#":root[data-theme="{name}"], :root:not([data-theme]) {{"#))) { continue ; } if noscript_css_line . starts_with (& end_theme_pat) && rustdoc_css_line . starts_with (& end_theme_pat) { break ; } if rustdoc_css_line != noscript_css_line { tidy_error ! (bad , "noscript.css:{noscript_css_line_number} and rustdoc.css:{rustdoc_css_line_number} contain copies of {name} theme that are not the same" ,) ; eprintln ! ("- {noscript_css_line}") ; eprintln ! ("+ {rustdoc_css_line}") ; return ; } } }
};
}
