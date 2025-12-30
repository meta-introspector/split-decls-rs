// Generated macro for QueryState (struct)
macro_rules! Depcrate_query_plumbingQueryState {
() => {
// Module: crate::query::plumbing
// Provides: {"QueryState"}
// Dependencies: {}
pub struct QueryState < K , I > { active : Sharded < hashbrown :: HashTable < (K , QueryResult < I >) > > , }
};
}
