// Generated macro for copy_recursive (function)
macro_rules! Depcratecopy_recursive {
() => {
// Module: crate
// Provides: {"copy_recursive"}
// Dependencies: {}
fn copy_recursive (from : & Path , to : & Path) { for entry in t ! (fs :: read_dir (from)) { let e = t ! (entry) ; let t = t ! (e . metadata ()) ; let dest = & to . join (e . file_name ()) ; if t . is_file () { t ! (fs :: copy (& e . path () , dest)) ; } else if t . is_dir () { t ! (fs :: create_dir_all (dest)) ; copy_recursive (& e . path () , dest) ; } } }
};
}
