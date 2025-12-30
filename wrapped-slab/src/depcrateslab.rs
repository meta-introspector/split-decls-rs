// Generated macro for Slab (struct)
macro_rules! DepcrateSlab {
() => {
// Module: crate
// Provides: {"Slab"}
// Dependencies: {}
# [doc = " Pre-allocated storage for a uniform data type"] # [doc = ""] # [doc = " See the [module documentation] for more details."] # [doc = ""] # [doc = " [module documentation]: index.html"] pub struct Slab < T > { entries : Vec < Entry < T > > , len : usize , next : usize , }
};
}
