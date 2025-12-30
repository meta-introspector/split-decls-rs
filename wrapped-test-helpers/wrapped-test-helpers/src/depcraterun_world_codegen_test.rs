// Generated macro for run_world_codegen_test (function)
macro_rules! Depcraterun_world_codegen_test {
() => {
// Module: crate
// Provides: {"run_world_codegen_test"}
// Dependencies: {}
pub fn run_world_codegen_test (gen_name : & str , wit_path : & Path , generate : fn (& Resolve , WorldId , & mut Files) , verify : fn (& Path , & str) ,) { let (resolve , world) = parse_wit (wit_path) ; let world_name = & resolve . worlds [world] . name ; let wit_name = if wit_path . is_dir () { wit_path . parent () . unwrap () . file_stem () . and_then (| s | s . to_str ()) . unwrap () } else { wit_path . file_stem () . and_then (| s | s . to_str ()) . unwrap () } ; let gen_name = format ! ("{gen_name}-{wit_name}") ; let dir = test_directory ("codegen" , & gen_name , & world_name) ; let mut files = Default :: default () ; generate (& resolve , world , & mut files) ; for (file , contents) in files . iter () { let dst = dir . join (file) ; std :: fs :: create_dir_all (dst . parent () . unwrap ()) . unwrap () ; std :: fs :: write (& dst , contents) . unwrap () ; } verify (& dir , & world_name) ; }
};
}
