// Generated macro for MAX_L2_SEARCH_MISSES (const)
macro_rules! Depcrate_byte_phf_builderMAX_L2_SEARCH_MISSES {
() => {
// Module: crate::byte_phf::builder
// Provides: {"MAX_L2_SEARCH_MISSES"}
// Dependencies: {}
# [doc = " To speed up the search algorithm, we limit the number of times the level-2 parameter (q)"] # [doc = " can hit its max value (initially Q_FAST_MAX) before we try the next level-1 parameter (p)."] # [doc = " In practice, this has a small impact on the resulting perfect hash, resulting in about"] # [doc = " 1 in 10000 hash maps that fall back to the slow path."] const MAX_L2_SEARCH_MISSES : usize = 24 ;
};
}
