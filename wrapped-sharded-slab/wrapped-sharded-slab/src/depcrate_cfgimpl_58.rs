// Generated macro for impl_58 (impl)
macro_rules! Depcrate_cfgimpl_58 {
() => {
// Module: crate::cfg
// Provides: {"impl_58"}
// Dependencies: {}
impl Config for DefaultConfig { const INITIAL_PAGE_SIZE : usize = 32 ; # [cfg (target_pointer_width = "64")] const MAX_THREADS : usize = 4096 ; # [cfg (target_pointer_width = "32")] const MAX_THREADS : usize = 128 ; const MAX_PAGES : usize = WIDTH / 2 ; }
};
}
