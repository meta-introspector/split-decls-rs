// Generated macro for gzip (function)
macro_rules! Depcrate_distgzip {
() => {
// Module: crate::dist
// Provides: {"gzip"}
// Dependencies: {}
fn gzip (src_path : & Path , dest_path : & Path) -> anyhow :: Result < () > { let mut encoder = GzEncoder :: new (File :: create (dest_path) ? , Compression :: best ()) ; let mut input = io :: BufReader :: new (File :: open (src_path) ?) ; io :: copy (& mut input , & mut encoder) ? ; encoder . finish () ? ; Ok (()) }
};
}
