// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () -> Result < () > { rayon :: ThreadPoolBuilder :: new () . num_threads (20) . build_global () ? ; let crate_dirs = find_crate_directories () ? ; println ! ("🚀 Processing {} crate directories with up to 20 crates in parallel" , crate_dirs . len ()) ; crate_dirs . par_iter () . for_each (| crate_dir | { if let Err (e) = split_crate (crate_dir) { println ! ("❌ Failed to process {}: {}" , crate_dir . display () , e) ; } }) ; println ! ("🎉 Done!") ; Ok (()) }
};
}
