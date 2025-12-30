// Generated macro for rustdoc_js_files (function)
macro_rules! Depcrate_extra_checks_rustdoc_jsrustdoc_js_files {
() => {
// Module: crate::extra_checks::rustdoc_js
// Provides: {"rustdoc_js_files"}
// Dependencies: {}
fn rustdoc_js_files (librustdoc_path : & Path) -> Vec < PathBuf > { let mut files = Vec :: new () ; walk_no_read (& [& librustdoc_path . join ("html/static/js")] , | path , is_dir | is_dir || path . extension () . is_none_or (| ext | ext != OsStr :: new ("js")) , & mut | path : & DirEntry | { files . push (path . path () . into ()) ; } ,) ; return files ; }
};
}
