// Generated macro for impl_29 (impl)
macro_rules! Depcrate_cartable_ptrimpl_29 {
() => {
// Module: crate::cartable_ptr
// Provides: {"impl_29"}
// Dependencies: {}
impl < C > CartableOptionPointer < C > where C : CartablePointerLike , { # [doc = " Creates a new instance corresponding to a `None` value."] # [inline] pub (crate) fn none () -> Self { Self { inner : sentinel_for :: < C :: Raw > () , _cartable : PhantomData , } } # [doc = " Creates a new instance corresponding to a `Some` value."] # [inline] pub (crate) fn from_cartable (cartable : C) -> Self { let inner = cartable . into_raw () ; debug_assert_ne ! (inner , sentinel_for ::< C :: Raw > ()) ; Self { inner , _cartable : PhantomData , } } # [doc = " Returns whether this instance is `None`. From the return value:"] # [doc = ""] # [doc = " - If `true`, the instance is `None`"] # [doc = " - If `false`, the instance is a valid `SelectedRc`"] # [inline] pub fn is_none (& self) -> bool { self . inner == sentinel_for :: < C :: Raw > () } }
};
}
