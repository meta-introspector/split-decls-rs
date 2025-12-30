// Generated macro for GetValues (trait)
macro_rules! Depcrate_unix_freebsd_diskGetValues {
() => {
// Module: crate::unix::freebsd::disk
// Provides: {"GetValues"}
// Dependencies: {}
trait GetValues { fn update_old (& mut self) ; fn get_read (& mut self) -> & mut u64 ; fn get_written (& mut self) -> & mut u64 ; fn dev_id (& self) -> Option < & String > ; }
};
}
