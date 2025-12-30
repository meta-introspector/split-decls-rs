// Generated macro for __internal (module)
macro_rules! Depcrate__internal {
() => {
// Module: crate
// Provides: {"__internal"}
// Dependencies: {}
# [doc = " Internal module used as support for `AssertZeroizeOnDrop`."] # [doc (hidden)] pub mod __internal { use super :: * ; # [doc = " Auto-deref workaround for deriving `ZeroizeOnDrop`."] pub trait AssertZeroizeOnDrop { fn zeroize_or_on_drop (self) ; } impl < T : ZeroizeOnDrop + ? Sized > AssertZeroizeOnDrop for & & mut T { fn zeroize_or_on_drop (self) { } } # [doc = " Auto-deref workaround for deriving `ZeroizeOnDrop`."] pub trait AssertZeroize { fn zeroize_or_on_drop (& mut self) ; } impl < T : Zeroize + ? Sized > AssertZeroize for T { fn zeroize_or_on_drop (& mut self) { self . zeroize () } } }
};
}
