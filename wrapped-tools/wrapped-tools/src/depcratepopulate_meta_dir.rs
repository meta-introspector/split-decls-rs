// Generated macro for populate_meta_dir (function)
macro_rules! Depcratepopulate_meta_dir {
() => {
// Module: crate
// Provides: {"populate_meta_dir"}
// Dependencies: {}
fn populate_meta_dir (destination_dir : & Path , script_identity : u32) -> std :: io :: Result < PathBuf > { let meta_dir = destination_dir . join (META_DIR_NAME) ; std :: fs :: create_dir_all (& meta_dir) ? ; std :: fs :: write (meta_dir . join (META_IDENTITY) , format ! ("{}-{}" , script_identity , family_name ()) . as_bytes () ,) ? ; std :: fs :: write (meta_dir . join (META_GIT_VERSION) , std :: process :: Command :: new (GIT_PROGRAM) . arg ("--version") . output () ? . stdout ,) ? ; Ok (meta_dir) }
};
}
