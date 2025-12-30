// Generated macro for exists (function)
macro_rules! Depcrate_sys_fs_commonexists {
() => {
// Module: crate::sys::fs::common
// Provides: {"exists"}
// Dependencies: {}
pub fn exists (path : & Path) -> io :: Result < bool > { match fs :: metadata (path) { Ok (_) => Ok (true) , Err (error) if error . kind () == io :: ErrorKind :: NotFound => Ok (false) , Err (error) => Err (error) , } }
};
}
