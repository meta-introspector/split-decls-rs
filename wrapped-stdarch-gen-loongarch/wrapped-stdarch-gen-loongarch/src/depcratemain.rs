// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
pub fn main () -> io :: Result < () > { let args : Vec < String > = env :: args () . collect () ; let in_file = args . get (1) . cloned () . expect ("Input file missing!") ; let in_file_path = PathBuf :: from (& in_file) ; let in_file_name = in_file_path . file_name () . unwrap () . to_os_string () . into_string () . unwrap () ; let ext_name = if in_file_name . starts_with ("lasx") { "lasx" } else { "lsx" } ; if in_file_name . ends_with (".h") { gen_spec (in_file , ext_name) } else if args . get (2) . is_some () { gen_test (in_file , ext_name) } else { gen_bind (in_file , ext_name) } }
};
}
