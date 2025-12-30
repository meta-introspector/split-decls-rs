// Generated macro for mdbook_test (function)
macro_rules! Depcratemdbook_test {
() => {
// Module: crate
// Provides: {"mdbook_test"}
// Dependencies: {}
fn mdbook_test () -> Result < () > { eprintln ! ("Testing inline code tests...") ; let status = Command :: new ("mdbook") . arg ("test") . status () . expect ("mdbook should be installed") ; if ! status . success () { return Err ("inline code tests failed" . into ()) ; } Ok (()) }
};
}
