// Generated macro for impl_35 (impl)
macro_rules! Depcrate_mapimpl_35 {
() => {
// Module: crate::map
// Provides: {"impl_35"}
// Dependencies: {}
impl < 'a , K : Ord , V > VacantEntry < 'a , K , V > { # [doc = " Gets a reference to the key that would be used when inserting a value"] # [doc = " through the `VacantEntry`."] # [inline] pub fn key (& self) -> & K { self . vacant . key () } # [doc = " Sets the value of the entry with the `VacantEntry`'s key, and returns a"] # [doc = " mutable reference to it."] # [inline] pub fn insert (self , value : V) -> & 'a mut V { self . vacant . insert (value) } }
};
}
