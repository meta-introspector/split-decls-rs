// Generated macro for run_test (function)
macro_rules! Depcraterun_test {
() => {
// Module: crate
// Provides: {"run_test"}
// Dependencies: {}
fn run_test (before : String , after : String , example : String , expect_success : bool , run_program : bool ,) -> Result < () , Box < dyn Error > > { let tempdir = tempfile :: TempDir :: new () ? ; let before_p = tempdir . path () . join ("before.rs") ; let after_p = tempdir . path () . join ("after.rs") ; let example_p = tempdir . path () . join ("example.rs") ; let check_fn = if run_program { run_check } else { compile_check } ; compile_check (before , & before_p , CRATE_NAME , false , true) ? ; check_fn (example . clone () , & example_p , "example" , true , true) ? ; compile_check (after , & after_p , CRATE_NAME , false , true) ? ; check_fn (example , & example_p , "example" , true , expect_success) ? ; Ok (()) }
};
}
