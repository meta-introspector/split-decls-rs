// Generated macro for ErasedBoxCart (type)
macro_rules! Depcrate_erasedErasedBoxCart {
() => {
// Module: crate::erased
// Provides: {"ErasedBoxCart"}
// Dependencies: {}
# [doc = " A type-erased Cart that has `Box` semantics"] # [doc = ""] # [doc = " See the docs of [`Yoke::erase_box_cart()`](crate::Yoke::erase_box_cart) for more info."] # [doc = ""] # [doc = " ✨ *Enabled with the `alloc` Cargo feature.*"] pub type ErasedBoxCart = Box < dyn ErasedDestructor > ;
};
}
