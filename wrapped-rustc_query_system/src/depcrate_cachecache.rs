// Generated macro for Cache (struct)
macro_rules! Depcrate_cacheCache {
() => {
// Module: crate::cache
// Provides: {"Cache"}
// Dependencies: {}
pub struct Cache < Key , Value > { hashmap : Lock < FxHashMap < Key , WithDepNode < Value > > > , }
};
}
