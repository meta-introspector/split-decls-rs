// Generated macro for impl_259 (impl)
macro_rules! Depcrate_metadataimpl_259 {
() => {
// Module: crate::metadata
// Provides: {"impl_259"}
// Dependencies: {}
impl PartialOrd for LevelFilter { # [inline (always)] fn partial_cmp (& self , other : & LevelFilter) -> Option < cmp :: Ordering > { Some (self . cmp (other)) } # [inline (always)] fn lt (& self , other : & LevelFilter) -> bool { filter_as_usize (& other . 0) < filter_as_usize (& self . 0) } # [inline (always)] fn le (& self , other : & LevelFilter) -> bool { filter_as_usize (& other . 0) <= filter_as_usize (& self . 0) } # [inline (always)] fn gt (& self , other : & LevelFilter) -> bool { filter_as_usize (& other . 0) > filter_as_usize (& self . 0) } # [inline (always)] fn ge (& self , other : & LevelFilter) -> bool { filter_as_usize (& other . 0) >= filter_as_usize (& self . 0) } }
};
}
