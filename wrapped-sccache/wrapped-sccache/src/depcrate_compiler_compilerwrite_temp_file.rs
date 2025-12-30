// Generated macro for write_temp_file (function)
macro_rules! Depcrate_compiler_compilerwrite_temp_file {
() => {
// Module: crate::compiler::compiler
// Provides: {"write_temp_file"}
// Dependencies: {}
# [doc = " Creates a future that will write `contents` to `path` inside of a temporary"] # [doc = " directory."] # [doc = ""] # [doc = " The future will resolve to the temporary directory and an absolute path"] # [doc = " inside that temporary directory with a file that has the same filename as"] # [doc = " `path` contains the `contents` specified."] # [doc = ""] # [doc = " Note that when the `TempDir` is dropped it will delete all of its contents"] # [doc = " including the path returned."] pub async fn write_temp_file (pool : & tokio :: runtime :: Handle , path : & Path , contents : Vec < u8 > ,) -> Result < (TempDir , PathBuf) > { let path = path . to_owned () ; pool . spawn_blocking (move | | { let dir = tempfile :: Builder :: new () . prefix ("sccache") . tempdir () ? ; let src = dir . path () . join (path) ; let mut file = File :: create (& src) ? ; file . write_all (& contents) ? ; Ok :: < _ , anyhow :: Error > ((dir , src)) }) . await ? . context ("failed to write temporary file") }
};
}
