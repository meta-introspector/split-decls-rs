// Generated macro for fmt (function)
macro_rules! Depcratefmt {
() => {
// Module: crate
// Provides: {"fmt"}
// Dependencies: {}
fn fmt () -> Result < () > { eprintln ! ("Checking code formatting...") ; for dir in ["style-check" , "mdbook-spec" , "xtask"] { let status = Command :: new ("cargo") . args (["fmt" , "--check"]) . current_dir (dir) . status () . expect ("cargo should be installed") ; if ! status . success () { return Err (format ! ("fmt check failed for {dir}") . into ()) ; } } Ok (()) }
};
}
