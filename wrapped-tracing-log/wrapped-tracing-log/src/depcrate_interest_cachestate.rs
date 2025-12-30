// Generated macro for State (struct)
macro_rules! Depcrate_interest_cacheState {
() => {
// Module: crate::interest_cache
// Provides: {"State"}
// Dependencies: {}
struct State { min_verbosity : Level , epoch : usize , cache : LruCache < Key , u64 , ahash :: RandomState > , }
};
}
