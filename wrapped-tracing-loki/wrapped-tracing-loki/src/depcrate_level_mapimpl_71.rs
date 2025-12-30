// Generated macro for impl_71 (impl)
macro_rules! Depcrate_level_mapimpl_71 {
() => {
// Module: crate::level_map
// Provides: {"impl_71"}
// Dependencies: {}
impl < T > ops :: IndexMut < Level > for LevelMap < T > { fn index_mut (& mut self , index : Level) -> & mut T { & mut self . map [level_index (index)] } }
};
}
