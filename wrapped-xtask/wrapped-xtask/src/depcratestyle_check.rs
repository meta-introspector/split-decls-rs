// Generated macro for style_check (function)
macro_rules! Depcratestyle_check {
() => {
// Module: crate
// Provides: {"style_check"}
// Dependencies: {}
fn style_check () -> Result < () > { eprintln ! ("Running style checks...") ; let status = Command :: new ("cargo") . args (["run" , "--manifest-path=style-check/Cargo.toml" , "--" , "src"]) . status () . expect ("cargo should be installed") ; if ! status . success () { return Err ("style check failed" . into ()) ; } Ok (()) }
};
}
