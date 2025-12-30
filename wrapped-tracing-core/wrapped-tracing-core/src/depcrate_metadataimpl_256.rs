// Generated macro for impl_256 (impl)
macro_rules! Depcrate_metadataimpl_256 {
() => {
// Module: crate::metadata
// Provides: {"impl_256"}
// Dependencies: {}
impl PartialOrd < LevelFilter > for Level { # [inline (always)] fn partial_cmp (& self , other : & LevelFilter) -> Option < cmp :: Ordering > { Some (filter_as_usize (& other . 0) . cmp (& (self . 0 as usize))) } # [inline (always)] fn lt (& self , other : & LevelFilter) -> bool { filter_as_usize (& other . 0) < (self . 0 as usize) } # [inline (always)] fn le (& self , other : & LevelFilter) -> bool { filter_as_usize (& other . 0) <= (self . 0 as usize) } # [inline (always)] fn gt (& self , other : & LevelFilter) -> bool { filter_as_usize (& other . 0) > (self . 0 as usize) } # [inline (always)] fn ge (& self , other : & LevelFilter) -> bool { filter_as_usize (& other . 0) >= (self . 0 as usize) } }
};
}
