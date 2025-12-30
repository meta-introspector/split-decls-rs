// Generated macro for compile_check (function)
macro_rules! Depcratecompile_check {
() => {
// Module: crate
// Provides: {"compile_check"}
// Dependencies: {}
fn compile_check (mut contents : String , path : & Path , crate_name : & str , extern_path : bool , expect_success : bool ,) -> Result < () , Box < dyn Error > > { let expected_error = match contents . find ("// Error:") { Some (index) => { let start = contents [.. index] . rfind (| ch | ch != ' ') . unwrap () ; let end = contents [index ..] . find ('\n') . unwrap () ; let error = contents [index + 9 .. index + end] . trim () . to_string () ; contents . replace_range (start + 1 .. index + end , "") ; Some (error) } None => None , } ; let output = compile (& contents , path , crate_name , extern_path) ? ; let stderr = std :: str :: from_utf8 (& output . stderr) . unwrap () ; match (output . status . success () , expect_success) { (true , true) => Ok (()) , (true , false) => Err (format ! ("expected failure, got success {}\n===== Contents:\n{}\n===== Output:\n{}\n" , path . display () , contents , stderr) . into ()) , (false , true) => Err (format ! ("expected success, got error {}\n===== Contents:\n{}\n===== Output:\n{}\n" , path . display () , contents , stderr) . into ()) , (false , false) => { if expected_error . is_none () { return Err ("failing test should have an \"// Error:\" annotation " . into ()) ; } let expected_error = expected_error . unwrap () ; if ! stderr . contains (& expected_error) { Err (format ! ("expected error message not found in compiler output\nExpected: {}\nGot:\n{}\n" , expected_error , stderr) . into ()) } else { Ok (()) } } } }
};
}
