// Generated macro for linkcheck (function)
macro_rules! Depcratelinkcheck {
() => {
// Module: crate
// Provides: {"linkcheck"}
// Dependencies: {}
fn linkcheck (args : impl Iterator < Item = String >) -> Result < () > { eprintln ! ("Running linkcheck...") ; let status = Command :: new ("curl") . args (["-sSLo" , "linkcheck.sh" , "https://raw.githubusercontent.com/rust-lang/rust/master/src/tools/linkchecker/linkcheck.sh"]) . status () . expect ("curl should be installed") ; if ! status . success () { return Err ("failed to fetch script from GitHub" . into ()) ; } let status = Command :: new ("sh") . args (["linkcheck.sh" , "--all" , "reference"]) . args (args) . status () . expect ("sh should be installed") ; if ! status . success () { return Err ("linkcheck failed" . into ()) ; } Ok (()) }
};
}
