// Generated macro for QueryCache (trait)
macro_rules! Depcrate_query_cachesQueryCache {
() => {
// Module: crate::query::caches
// Provides: {"QueryCache"}
// Dependencies: {}
# [doc = " Trait for types that serve as an in-memory cache for query results,"] # [doc = " for a given key (argument) type and value (return) type."] # [doc = ""] # [doc = " Types implementing this trait are associated with actual key/value types"] # [doc = " by the `Cache` associated type of the `rustc_middle::query::Key` trait."] pub trait QueryCache : Sized { type Key : Hash + Eq + Copy + Debug ; type Value : Copy ; # [doc = " Returns the cached value (and other information) associated with the"] # [doc = " given key, if it is present in the cache."] fn lookup (& self , key : & Self :: Key) -> Option < (Self :: Value , DepNodeIndex) > ; # [doc = " Adds a key/value entry to this cache."] # [doc = ""] # [doc = " Called by some part of the query system, after having obtained the"] # [doc = " value by executing the query or loading a cached value from disk."] fn complete (& self , key : Self :: Key , value : Self :: Value , index : DepNodeIndex) ; fn iter (& self , f : & mut dyn FnMut (& Self :: Key , & Self :: Value , DepNodeIndex)) ; }
};
}
