// Generated macro for get_all_files (function)
macro_rules! Depcrate_lru_disk_cacheget_all_files {
() => {
// Module: crate::lru_disk_cache
// Provides: {"get_all_files"}
// Dependencies: {}
# [doc = " Return an iterator of `(path, size)` of files under `path` sorted by ascending last-modified"] # [doc = " time, such that the oldest modified file is returned first."] fn get_all_files < P : AsRef < Path > > (path : P) -> Box < dyn Iterator < Item = (PathBuf , u64) > > { let mut files : Vec < _ > = WalkDir :: new (path . as_ref ()) . into_iter () . filter_map (| e | { e . ok () . and_then (| f | { if f . file_type () . is_file () { f . metadata () . ok () . and_then (| m | { m . modified () . ok () . map (| mtime | (mtime , f . path () . to_owned () , m . len ())) }) } else { None } }) }) . collect () ; files . sort_by_key (| k | k . 0) ; Box :: new (files . into_iter () . map (| (_mtime , path , size) | (path , size))) }
};
}
