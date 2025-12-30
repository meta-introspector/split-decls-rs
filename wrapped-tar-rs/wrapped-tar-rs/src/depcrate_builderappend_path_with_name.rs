// Generated macro for append_path_with_name (function)
macro_rules! Depcrate_builderappend_path_with_name {
() => {
// Module: crate::builder
// Provides: {"append_path_with_name"}
// Dependencies: {}
fn append_path_with_name (dst : & mut dyn Write , path : & Path , name : Option < & Path > , options : BuilderOptions ,) -> io :: Result < () > { let stat = if options . follow { fs :: metadata (path) . map_err (| err | { io :: Error :: new (err . kind () , format ! ("{} when getting metadata for {}" , err , path . display ()) ,) }) ? } else { fs :: symlink_metadata (path) . map_err (| err | { io :: Error :: new (err . kind () , format ! ("{} when getting metadata for {}" , err , path . display ()) ,) }) ? } ; let ar_name = name . unwrap_or (path) ; if stat . is_file () { append_file (dst , ar_name , & mut fs :: File :: open (path) ? , options) } else if stat . is_dir () { append_fs (dst , ar_name , & stat , options . mode , None) } else if stat . file_type () . is_symlink () { let link_name = fs :: read_link (path) ? ; append_fs (dst , ar_name , & stat , options . mode , Some (& link_name)) } else { # [cfg (unix)] { append_special (dst , path , & stat , options . mode) } # [cfg (not (unix))] { Err (other (& format ! ("{} has unknown file type" , path . display ()))) } } }
};
}
