// Generated macro for Error (enum)
macro_rules! Depcrate_lru_disk_cacheError {
() => {
// Module: crate::lru_disk_cache
// Provides: {"Error"}
// Dependencies: {}
# [doc = " Errors returned by this crate."] # [derive (Debug)] pub enum Error { # [doc = " The file was too large to fit in the cache."] FileTooLarge , # [doc = " The file was not in the cache."] FileNotInCache , # [doc = " An IO Error occurred."] Io (io :: Error) , }
};
}
