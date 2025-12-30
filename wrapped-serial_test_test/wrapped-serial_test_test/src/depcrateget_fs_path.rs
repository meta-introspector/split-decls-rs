// Generated macro for get_fs_path (function)
macro_rules! Depcrateget_fs_path {
() => {
// Module: crate
// Provides: {"get_fs_path"}
// Dependencies: {}
fn get_fs_path () -> PathBuf { static FS_PATH : OnceCell < PathBuf > = OnceCell :: new () ; FS_PATH . get_or_init (env :: temp_dir) . clone () }
};
}
