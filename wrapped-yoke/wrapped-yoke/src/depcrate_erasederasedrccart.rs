// Generated macro for ErasedRcCart (type)
macro_rules! Depcrate_erasedErasedRcCart {
() => {
// Module: crate::erased
// Provides: {"ErasedRcCart"}
// Dependencies: {}
# [doc = " A type-erased Cart that has `Rc` semantics"] # [doc = ""] # [doc = " See the docs of [`Yoke::erase_rc_cart()`](crate::Yoke::erase_rc_cart) for more info."] # [doc = ""] # [doc = " ✨ *Enabled with the `alloc` Cargo feature.*"] pub type ErasedRcCart = Rc < dyn ErasedDestructor > ;
};
}
