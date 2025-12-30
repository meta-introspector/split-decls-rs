// Generated macro for impl_124 (impl)
macro_rules! Depcrate_yokeimpl_124 {
() => {
// Module: crate::yoke
// Provides: {"impl_124"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < Y : for < 'a > Yokeable < 'a > , C > Yoke < Y , C > { # [doc = " Helper function allowing one to wrap the cart type `C` in a `Box<T>`."] # [doc = " Can be paired with [`Yoke::erase_box_cart()`]"] # [doc = ""] # [doc = " ✨ *Enabled with the `alloc` Cargo feature.*"] # [inline] pub fn wrap_cart_in_box (self) -> Yoke < Y , Box < C > > { unsafe { self . replace_cart (Box :: new) } } # [doc = " Helper function allowing one to wrap the cart type `C` in an `Rc<T>`."] # [doc = " Can be paired with [`Yoke::erase_rc_cart()`], or generally used"] # [doc = " to make the [`Yoke`] cloneable."] # [doc = ""] # [doc = " ✨ *Enabled with the `alloc` Cargo feature.*"] # [inline] pub fn wrap_cart_in_rc (self) -> Yoke < Y , Rc < C > > { unsafe { self . replace_cart (Rc :: new) } } # [doc = " Helper function allowing one to wrap the cart type `C` in an `Arc<T>`."] # [doc = " Can be paired with [`Yoke::erase_arc_cart()`], or generally used"] # [doc = " to make the [`Yoke`] cloneable."] # [doc = ""] # [doc = " ✨ *Enabled with the `alloc` Cargo feature.*"] # [inline] pub fn wrap_cart_in_arc (self) -> Yoke < Y , Arc < C > > { unsafe { self . replace_cart (Arc :: new) } } }
};
}
