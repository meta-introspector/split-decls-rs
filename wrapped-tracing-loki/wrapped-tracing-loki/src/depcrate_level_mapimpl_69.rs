// Generated macro for impl_69 (impl)
macro_rules! Depcrate_level_mapimpl_69 {
() => {
// Module: crate::level_map
// Provides: {"impl_69"}
// Dependencies: {}
impl < T : fmt :: Debug > fmt :: Debug for LevelMap < T > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . debug_map () . entry (& Level :: TRACE , & self [Level :: TRACE]) . entry (& Level :: DEBUG , & self [Level :: DEBUG]) . entry (& Level :: INFO , & self [Level :: INFO]) . entry (& Level :: WARN , & self [Level :: WARN]) . entry (& Level :: ERROR , & self [Level :: ERROR]) . finish () } }
};
}
