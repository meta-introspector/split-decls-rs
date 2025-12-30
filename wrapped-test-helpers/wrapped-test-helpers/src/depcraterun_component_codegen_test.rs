// Generated macro for run_component_codegen_test (function)
macro_rules! Depcraterun_component_codegen_test {
() => {
// Module: crate
// Provides: {"run_component_codegen_test"}
// Dependencies: {}
pub fn run_component_codegen_test (gen_name : & str , wit_path : & Path , generate : fn (& str , & [u8] , & mut Files) , verify : fn (& Path , & str) ,) { let (resolve , world) = parse_wit (wit_path) ; let world_name = & resolve . worlds [world] . name ; let mut wasm = wit_component :: dummy_module (& resolve , world , ManglingAndAbi :: Standard32) ; let encoded = wit_component :: metadata :: encode (& resolve , world , StringEncoding :: UTF8 , None) . unwrap () ; let section = wasm_encoder :: CustomSection { name : std :: borrow :: Cow :: Borrowed ("component-type") , data : std :: borrow :: Cow :: Borrowed (& encoded) , } ; wasm . push (section . id ()) ; section . encode (& mut wasm) ; let component = wit_component :: ComponentEncoder :: default () . module (& wasm) . unwrap () . validate (true) . encode () . unwrap () ; let wit_name = if wit_path . is_dir () { wit_path . parent () . unwrap () . file_stem () . and_then (| s | s . to_str ()) . unwrap () } else { wit_path . file_stem () . and_then (| s | s . to_str ()) . unwrap () } ; let gen_name = format ! ("{gen_name}-{wit_name}" ,) ; let dir = test_directory ("codegen" , & gen_name , & world_name) ; std :: fs :: write (dir . join ("component.wasm") , & component) . unwrap () ; let mut files = Default :: default () ; generate (& world_name , & component , & mut files) ; for (file , contents) in files . iter () { let dst = dir . join (file) ; std :: fs :: create_dir_all (dst . parent () . unwrap ()) . unwrap () ; std :: fs :: write (& dst , contents) . unwrap () ; } verify (& dir , & world_name) ; }
};
}
