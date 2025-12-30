// Generated macro for compute_bcp47_ids_hash (function)
macro_rules! Depcrate_time_zones_namescompute_bcp47_ids_hash {
() => {
// Module: crate::time_zones::names
// Provides: {"compute_bcp47_ids_hash"}
// Dependencies: {}
fn compute_bcp47_ids_hash (bcp47_ids : & Vec < TimeZone >) -> u64 { let mut hasher = create_hasher () ; bcp47_ids . hash (& mut hasher) ; hasher . finish () }
};
}
