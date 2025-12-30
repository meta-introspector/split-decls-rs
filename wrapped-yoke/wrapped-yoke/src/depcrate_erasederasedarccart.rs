// Generated macro for ErasedArcCart (type)
macro_rules! Depcrate_erasedErasedArcCart {
() => {
// Module: crate::erased
// Provides: {"ErasedArcCart"}
// Dependencies: {}
# [doc = " A type-erased Cart that has `Arc` semantics"] # [doc = ""] # [doc = " See the docs of [`Yoke::erase_arc_cart()`](crate::Yoke::erase_rc_cart) for more info."] # [doc = ""] # [doc = " ✨ *Enabled with the `alloc` Cargo feature.*"] pub type ErasedArcCart = Arc < dyn ErasedDestructor + Send + Sync > ;
};
}
