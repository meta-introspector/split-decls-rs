// Generated macro for load_port_file (function)
macro_rules! Depcrateload_port_file {
() => {
// Module: crate
// Provides: {"load_port_file"}
// Dependencies: {}
fn load_port_file (filename : & PathBuf , port_info : & mut Vec < BTreeMap < String , String > > ,) -> Result < () , Error > { let f = try ! (File :: open (& filename) . map_err (| e | Error :: VcpkgInstallation (format ! ("Could not open status file at {}: {}" , filename . display () , e)))) ; let file = BufReader :: new (& f) ; let mut current : BTreeMap < String , String > = BTreeMap :: new () ; for line in file . lines () { let line = line . unwrap () ; let parts = line . splitn (2 , ": ") . clone () . collect :: < Vec < _ > > () ; if parts . len () == 2 { current . insert (parts [0] . trim () . into () , parts [1] . trim () . into ()) ; } else if line . len () == 0 { port_info . push (current . clone ()) ; current . clear () ; } else { } } if ! current . is_empty () { port_info . push (current) ; } Ok (()) }
};
}
