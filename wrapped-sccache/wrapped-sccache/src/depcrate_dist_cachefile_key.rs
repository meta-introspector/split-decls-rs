// Generated macro for file_key (function)
macro_rules! Depcrate_dist_cachefile_key {
() => {
// Module: crate::dist::cache
// Provides: {"file_key"}
// Dependencies: {}
fn file_key < R : Read > (rdr : R) -> Result < String > { Digest :: reader_sync (rdr) }
};
}
