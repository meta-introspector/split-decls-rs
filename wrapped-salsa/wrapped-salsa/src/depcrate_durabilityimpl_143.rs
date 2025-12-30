// Generated macro for impl_143 (impl)
macro_rules! Depcrate_durabilityimpl_143 {
() => {
// Module: crate::durability
// Provides: {"impl_143"}
// Dependencies: {}
impl Durability { # [doc = " Low durability: things that change frequently."] # [doc = ""] # [doc = " Example: part of the crate being edited"] pub const LOW : Durability = Durability (DurabilityVal :: Low) ; # [doc = " Medium durability: things that change sometimes, but rarely."] # [doc = ""] # [doc = " Example: a Cargo.toml file"] pub const MEDIUM : Durability = Durability (DurabilityVal :: Medium) ; # [doc = " High durability: things that are not expected to change under"] # [doc = " common usage."] # [doc = ""] # [doc = " Example: the standard library or something from crates.io"] pub const HIGH : Durability = Durability (DurabilityVal :: High) ; # [doc = " The minimum possible durability; equivalent to LOW but"] # [doc = " \"conceptually\" distinct (i.e., if we add more durability"] # [doc = " levels, this could change)."] pub (crate) const MIN : Durability = Self :: LOW ; # [doc = " The maximum possible durability; equivalent to HIGH but"] # [doc = " \"conceptually\" distinct (i.e., if we add more durability"] # [doc = " levels, this could change)."] pub (crate) const MAX : Durability = Self :: HIGH ; # [doc = " Number of durability levels."] pub (crate) const LEN : usize = Self :: HIGH . 0 as usize + 1 ; pub (crate) fn index (self) -> usize { self . 0 as usize } }
};
}
