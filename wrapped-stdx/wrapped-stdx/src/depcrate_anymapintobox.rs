// Generated macro for IntoBox (trait)
macro_rules! Depcrate_anymapIntoBox {
() => {
// Module: crate::anymap
// Provides: {"IntoBox"}
// Dependencies: {}
# [doc = " A trait for the conversion of an object into a boxed trait object."] pub trait IntoBox < A : ? Sized + Downcast > : Any { # [doc = " Convert self into the appropriate boxed form."] fn into_box (self) -> Box < A > ; }
};
}
