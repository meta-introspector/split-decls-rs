// Generated macro for handle_result (function)
macro_rules! Depcratehandle_result {
() => {
// Module: crate
// Provides: {"handle_result"}
// Dependencies: {}
fn handle_result (result : io :: Result < ExitStatus > , cmd : Command) { match result { Err (error) => { eprintln ! ("Failed to invoke `{cmd:?}`: {error}") ; } Ok (status) => { process :: exit (status . code () . unwrap_or (1)) ; } } }
};
}
