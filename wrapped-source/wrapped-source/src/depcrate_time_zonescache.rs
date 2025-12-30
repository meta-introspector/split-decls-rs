// Generated macro for Cache (type)
macro_rules! Depcrate_time_zonesCache {
() => {
// Module: crate::time_zones
// Provides: {"Cache"}
// Dependencies: {}
type Cache < T > = OnceLock < Result < T , DataError > > ;
};
}
