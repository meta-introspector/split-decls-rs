// Generated macro for impl_70 (impl)
macro_rules! Depcrate_level_mapimpl_70 {
() => {
// Module: crate::level_map
// Provides: {"impl_70"}
// Dependencies: {}
impl < T > ops :: Index < Level > for LevelMap < T > { type Output = T ; fn index (& self , index : Level) -> & T { & self . map [level_index (index)] } }
};
}
