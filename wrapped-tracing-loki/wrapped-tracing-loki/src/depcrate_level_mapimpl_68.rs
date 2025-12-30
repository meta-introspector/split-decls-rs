// Generated macro for impl_68 (impl)
macro_rules! Depcrate_level_mapimpl_68 {
() => {
// Module: crate::level_map
// Provides: {"impl_68"}
// Dependencies: {}
impl < T > LevelMap < T > { pub fn from_fn < F : FnMut (Level) -> T > (mut f : F) -> LevelMap < T > { LevelMap { map : [f (Level :: TRACE) , f (Level :: DEBUG) , f (Level :: INFO) , f (Level :: WARN) , f (Level :: ERROR) ,] , } } pub fn values (& self) -> slice :: Iter < '_ , T > { self . map . iter () } pub fn values_mut (& mut self) -> slice :: IterMut < '_ , T > { self . map . iter_mut () } }
};
}
