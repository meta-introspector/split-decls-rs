// Generated macro for hash_result (function)
macro_rules! Depcrate_dep_graph_graphhash_result {
() => {
// Module: crate::dep_graph::graph
// Provides: {"hash_result"}
// Dependencies: {}
pub fn hash_result < R > (hcx : & mut StableHashingContext < '_ > , result : & R) -> Fingerprint where R : for < 'a > HashStable < StableHashingContext < 'a > > , { let mut stable_hasher = StableHasher :: new () ; result . hash_stable (hcx , & mut stable_hasher) ; stable_hasher . finish () }
};
}
