// Generated macro for impl_115 (impl)
macro_rules! Depcrate_ci_uhyveimpl_115 {
() => {
// Module: crate::ci::uhyve
// Provides: {"impl_115"}
// Dependencies: {}
impl Uhyve { pub fn run (self , image : & Path , smp : usize) -> Result < () > { let sh = crate :: sh () ? ; let uhyve = env :: var ("UHYVE") . unwrap_or_else (| _ | "uhyve" . to_string ()) ; let program = if self . sudo { "sudo" } else { uhyve . as_str () } ; let arg = self . sudo . then_some (uhyve . as_str ()) ; let smp_arg = format ! ("--cpu-count={smp}") ; cmd ! (sh , "{program} {arg...} {smp_arg} {image}") . env ("RUST_LOG" , "debug") . run () ? ; Ok (()) } }
};
}
