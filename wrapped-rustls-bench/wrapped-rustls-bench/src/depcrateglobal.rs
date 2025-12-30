// Generated macro for GLOBAL (static)
macro_rules! DepcrateGLOBAL {
() => {
// Module: crate
// Provides: {"GLOBAL"}
// Dependencies: {}
# [cfg (not (target_env = "msvc"))] # [global_allocator] static GLOBAL : tikv_jemallocator :: Jemalloc = tikv_jemallocator :: Jemalloc ;
};
}
