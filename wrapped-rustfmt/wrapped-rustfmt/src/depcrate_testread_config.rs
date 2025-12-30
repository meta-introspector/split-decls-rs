// Generated macro for read_config (function)
macro_rules! Depcrate_testread_config {
() => {
// Module: crate::test
// Provides: {"read_config"}
// Dependencies: {}
fn read_config (filename : & Path) -> Config { let sig_comments = read_significant_comments (filename) ; let (edition , style_edition , version) = get_editions_from_comments (& sig_comments) ; let mut config = if ! sig_comments . is_empty () { get_config (sig_comments . get ("config") . map (Path :: new) , edition , style_edition , version ,) } else { get_config (filename . with_extension ("toml") . file_name () . map (Path :: new) , edition , style_edition , version ,) } ; for (key , val) in & sig_comments { if key != "target" && key != "config" && key != "unstable" { config . override_value (key , val) ; if config . is_default (key) { warn ! ("Default value {} used explicitly for {}" , val , key) ; } } } config }
};
}
