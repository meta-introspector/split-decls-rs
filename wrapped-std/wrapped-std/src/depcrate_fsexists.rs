// Generated macro for exists (function)
macro_rules! Depcrate_fsexists {
() => {
// Module: crate::fs
// Provides: {"exists"}
// Dependencies: {}
# [doc = " Returns `Ok(true)` if the path points at an existing entity."] # [doc = ""] # [doc = " This function will traverse symbolic links to query information about the"] # [doc = " destination file. In case of broken symbolic links this will return `Ok(false)`."] # [doc = ""] # [doc = " As opposed to the [`Path::exists`] method, this will only return `Ok(true)` or `Ok(false)`"] # [doc = " if the path was _verified_ to exist or not exist. If its existence can neither be confirmed"] # [doc = " nor denied, an `Err(_)` will be propagated instead. This can be the case if e.g. listing"] # [doc = " permission is denied on one of the parent directories."] # [doc = ""] # [doc = " Note that while this avoids some pitfalls of the `exists()` method, it still can not"] # [doc = " prevent time-of-check to time-of-use ([TOCTOU]) bugs. You should only use it in scenarios"] # [doc = " where those bugs are not an issue."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```no_run"] # [doc = " use std::fs;"] # [doc = ""] # [doc = " assert!(!fs::exists(\"does_not_exist.txt\").expect(\"Can't check existence of file does_not_exist.txt\"));"] # [doc = " assert!(fs::exists(\"/root/secret_file.txt\").is_err());"] # [doc = " ```"] # [doc = ""] # [doc = " [`Path::exists`]: crate::path::Path::exists"] # [doc = " [TOCTOU]: self#time-of-check-to-time-of-use-toctou"] # [stable (feature = "fs_try_exists" , since = "1.81.0")] # [inline] pub fn exists < P : AsRef < Path > > (path : P) -> io :: Result < bool > { fs_imp :: exists (path . as_ref ()) }
};
}
