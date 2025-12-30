// Generated macro for verify_config_used (function)
macro_rules! Depcrate_testverify_config_used {
() => {
// Module: crate::test
// Provides: {"verify_config_used"}
// Dependencies: {}
fn verify_config_used (path : & Path , config_name : & str) { for entry in fs :: read_dir (path) . expect (& format ! ("couldn't read {} directory" , path . display ())) { let entry = entry . expect ("couldn't get directory entry") ; let path = entry . path () ; if path . extension () . map_or (false , | f | f == "rs") { let filebuf = BufReader :: new (fs :: File :: open (& path) . unwrap_or_else (| _ | panic ! ("couldn't read file {}" , path . display ())) ,) ; assert ! (filebuf . lines () . map (Result :: unwrap) . take_while (| l | l . starts_with ("//")) . any (| l | l . starts_with (& format ! ("// rustfmt-{}" , config_name))) , "config option file {} does not contain expected config name" , path . display ()) ; } } }
};
}
