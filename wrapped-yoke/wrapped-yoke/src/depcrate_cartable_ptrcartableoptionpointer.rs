// Generated macro for CartableOptionPointer (struct)
macro_rules! Depcrate_cartable_ptrCartableOptionPointer {
() => {
// Module: crate::cartable_ptr
// Provides: {"CartableOptionPointer"}
// Dependencies: {}
# [doc = " A type with similar semantics as `Option<C<T>>` but with a niche."] # [doc = ""] # [doc = " This type cannot be publicly constructed. To use this in a `Yoke`, see"] # [doc = " [`Yoke::convert_cart_into_option_pointer`]."] # [doc = ""] # [doc = " [`Yoke::convert_cart_into_option_pointer`]: crate::Yoke::convert_cart_into_option_pointer"] # [derive (Debug)] pub struct CartableOptionPointer < C > where C : CartablePointerLike , { # [doc = " The inner pointer."] # [doc = ""] # [doc = " # Invariants"] # [doc = ""] # [doc = " 1. Must be either `SENTINEL_PTR` or created from `CartablePointerLike::into_raw`"] # [doc = " 2. If non-sentinel, must _always_ be for a valid SelectedRc"] inner : NonNull < C :: Raw > , _cartable : PhantomData < C > , }
};
}
