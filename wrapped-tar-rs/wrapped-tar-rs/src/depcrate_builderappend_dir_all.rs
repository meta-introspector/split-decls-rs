// Generated macro for append_dir_all (function)
macro_rules! Depcrate_builderappend_dir_all {
() => {
// Module: crate::builder
// Provides: {"append_dir_all"}
// Dependencies: {}
fn append_dir_all (dst : & mut dyn Write , path : & Path , src_path : & Path , options : BuilderOptions ,) -> io :: Result < () > { let mut stack = vec ! [(src_path . to_path_buf () , true , false)] ; while let Some ((src , is_dir , is_symlink)) = stack . pop () { let dest = path . join (src . strip_prefix (src_path) . unwrap ()) ; if is_dir || (is_symlink && options . follow && src . is_dir ()) { for entry in fs :: read_dir (& src) ? { let entry = entry ? ; let file_type = entry . file_type () ? ; stack . push ((entry . path () , file_type . is_dir () , file_type . is_symlink ())) ; } if dest != Path :: new ("") { append_dir (dst , & dest , & src , options) ? ; } } else if ! options . follow && is_symlink { let stat = fs :: symlink_metadata (& src) ? ; let link_name = fs :: read_link (& src) ? ; append_fs (dst , & dest , & stat , options . mode , Some (& link_name)) ? ; } else { # [cfg (unix)] { let stat = fs :: metadata (& src) ? ; if ! stat . is_file () { append_special (dst , & dest , & stat , options . mode) ? ; continue ; } } append_file (dst , & dest , & mut fs :: File :: open (src) ? , options) ? ; } } Ok (()) }
};
}
