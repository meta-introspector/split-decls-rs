// Generated macro for A (static)
macro_rules! DepcrateA {
() => {
// Module: crate
// Provides: {"A"}
// Dependencies: {}
# [cfg (all (test , feature = "gg-alloc"))] # [global_allocator] static A : gg_alloc :: GgAlloc < std :: alloc :: System > = gg_alloc :: GgAlloc :: new (std :: alloc :: System) ;
};
}
