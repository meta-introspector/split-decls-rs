// Generated macro for create_writer (function)
macro_rules! Depcrate_rollingcreate_writer {
() => {
// Module: crate::rolling
// Provides: {"create_writer"}
// Dependencies: {}
fn create_writer (directory : & Path , filename : & str) -> Result < File , InitError > { let path = directory . join (filename) ; let mut open_options = OpenOptions :: new () ; open_options . append (true) . create (true) ; let new_file = open_options . open (path . as_path ()) ; if new_file . is_err () { if let Some (parent) = path . parent () { fs :: create_dir_all (parent) . map_err (InitError :: ctx ("failed to create log directory")) ? ; return open_options . open (path) . map_err (InitError :: ctx ("failed to create initial log file")) ; } } new_file . map_err (InitError :: ctx ("failed to create initial log file")) }
};
}
