// Generated macro for check_formatting (function)
macro_rules! Depcratecheck_formatting {
() => {
// Module: crate
// Provides: {"check_formatting"}
// Dependencies: {}
fn check_formatting (path : & Path) -> Result < () , Box < dyn Error > > { match Command :: new ("rustfmt") . args (& ["--edition=2018" , "--check"]) . arg (path) . status () { Ok (status) => { if ! status . success () { return Err (format ! ("failed to run rustfmt: {}" , status) . into ()) ; } Ok (()) } Err (e) => Err (format ! ("failed to run rustfmt: {}" , e) . into ()) , } }
};
}
