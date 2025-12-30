// Generated macro for generate_load_store_tests (function)
macro_rules! Depcrate_load_store_testsgenerate_load_store_tests {
() => {
// Module: crate::load_store_tests
// Provides: {"generate_load_store_tests"}
// Dependencies: {}
# [doc = " `load_intrinsics` and `store_intrinsics` is a vector of intrinsics"] # [doc = " variants, while `out_path` is a file to write to."] pub fn generate_load_store_tests (load_intrinsics : Vec < Intrinsic > , store_intrinsics : Vec < Intrinsic > , out_path : Option < & PathBuf > ,) -> Result < () , String > { let output = match out_path { Some (out) => { Box :: new (File :: create (out) . map_err (| e | format ! ("couldn't create tests file: {e}")) ?) as Box < dyn Write > } None => Box :: new (std :: io :: stdout ()) as Box < dyn Write > , } ; let mut used_stores = vec ! [false ; store_intrinsics . len ()] ; let tests : Vec < _ > = load_intrinsics . iter () . map (| load | { let store_candidate = load . signature . fn_name () . to_string () . replace ("svld1s" , "svst1") . replace ("svld1u" , "svst1") . replace ("svldnt1s" , "svstnt1") . replace ("svldnt1u" , "svstnt1") . replace ("svld" , "svst") . replace ("gather" , "scatter") ; let store_index = store_intrinsics . iter () . position (| i | i . signature . fn_name () . to_string () == store_candidate) ; if let Some (i) = store_index { used_stores [i] = true ; } generate_single_test (load . clone () , store_index . map (| i | store_intrinsics [i] . clone ()) ,) }) . try_collect () ? ; assert ! (used_stores . into_iter () . all (| b | b) , "Not all store tests have been paired with a load. Consider generating specifc store-only tests") ; let preamble = TokenStream :: from_str (& PREAMBLE) . map_err (| e | format ! ("Preamble is invalid: {e}")) ? ; let manual_tests = match & load_intrinsics [0] . target_features [..] { [s] if s == "sve" => TokenStream :: from_str (MANUAL_TESTS) . map_err (| e | format ! ("Manual tests are invalid: {e}")) ? , _ => quote ! () , } ; format_code (output , format ! ("// This code is automatically generated. DO NOT MODIFY.
//
// Instead, modify `crates/stdarch-gen-arm/spec/sve` and run the following command to re-generate
// this file:
//
// ```
// cargo run --bin=stdarch-gen-arm -- crates/stdarch-gen-arm/spec
// ```
{}" , quote ! { # preamble # (# tests) * # manual_tests }) ,) . map_err (| e | format ! ("couldn't write tests: {e}")) }
};
}
