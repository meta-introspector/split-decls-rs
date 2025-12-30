// Generated macro for DiffOpt (struct)
macro_rules! DepcrateDiffOpt {
() => {
// Module: crate
// Provides: {"DiffOpt"}
// Dependencies: {}
# [derive (Parser , Debug)] struct DiffOpt { base : PathBuf , change : PathBuf , # [arg (short = 'e' , long = "exclude")] exclude : Vec < String > , # [arg (long = "json")] json : bool , }
};
}
