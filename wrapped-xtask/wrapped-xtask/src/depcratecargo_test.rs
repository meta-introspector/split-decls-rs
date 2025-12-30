// Generated macro for cargo_test (function)
macro_rules! Depcratecargo_test {
() => {
// Module: crate
// Provides: {"cargo_test"}
// Dependencies: {}
fn cargo_test () -> Result < () > { eprintln ! ("Running cargo tests...") ; let status = Command :: new ("cargo") . arg ("test") . current_dir ("mdbook-spec") . status () . expect ("cargo should be installed") ; if ! status . success () { return Err ("mdbook-spec test failed" . into ()) ; } Ok (()) }
};
}
