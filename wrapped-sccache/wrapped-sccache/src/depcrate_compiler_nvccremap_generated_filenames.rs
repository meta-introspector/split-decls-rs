// Generated macro for remap_generated_filenames (function)
macro_rules! Depcrate_compiler_nvccremap_generated_filenames {
() => {
// Module: crate::compiler::nvcc
// Provides: {"remap_generated_filenames"}
// Dependencies: {}
fn remap_generated_filenames (args : & [String] , old_to_new : & mut HashMap < String , String > , ext_counts : & mut HashMap < String , i32 > ,) -> Vec < String > { args . iter () . map (| arg | { let arg_is_msvc_preprocessor_output = arg . starts_with ("-Fi") ; let arg = if arg_is_msvc_preprocessor_output { arg . trim_start_matches ("-Fi") . to_owned () } else { arg . to_owned () } ; let maybe_extension = if ! arg . starts_with ('-') { { [".cpp1.ii" , ".cpp4.ii" , ".cudafe1.c" , ".cudafe1.cpp" , ".cudafe1.stub.c" ,] . iter () . find (| ext | arg . ends_with (* ext)) . copied () } } else { None } ; let arg = match maybe_extension { Some (extension) => { old_to_new . entry (arg) . or_insert_with_key (| arg | { let count = ext_counts . entry (extension . into ()) . and_modify (| c | * c += 1) . or_insert (0) . to_string () ; PathBuf :: from (arg) . parent () . unwrap_or (Path :: new ("")) . join ("x_" . to_owned () + & count + extension) . to_string_lossy () . to_string () }) . to_owned () } None => { let mut arg = arg . clone () ; for (old , new) in old_to_new . iter () . sorted_by (| a , b | b . 0 . len () . cmp (& a . 0 . len ())) { arg = arg . replace (old , new) ; } arg } } ; if arg_is_msvc_preprocessor_output { format ! ("-Fi{}" , arg) } else { arg } }) . collect :: < Vec < _ > > () }
};
}
