// Generated macro for map_try_reserve_error (function)
macro_rules! Depcrate_collections_hash_mapmap_try_reserve_error {
() => {
// Module: crate::collections::hash::map
// Provides: {"map_try_reserve_error"}
// Dependencies: {}
# [inline] pub (super) fn map_try_reserve_error (err : hashbrown :: TryReserveError) -> TryReserveError { match err { hashbrown :: TryReserveError :: CapacityOverflow => { TryReserveErrorKind :: CapacityOverflow . into () } hashbrown :: TryReserveError :: AllocError { layout } => { TryReserveErrorKind :: AllocError { layout , non_exhaustive : () } . into () } } }
};
}
