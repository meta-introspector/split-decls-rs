// Generated macro for macro_103 (macro)
macro_rules! Depcrate_builtinmacro_103 {
() => {
// Module: crate::builtin
// Provides: {"macro_103"}
// Dependencies: {}
declare_lint ! { # [doc = " The `unused_associated_type_bounds` lint is emitted when an"] # [doc = " associated type bound is added to a trait object, but the associated"] # [doc = " type has a `where Self: Sized` bound, and is thus unavailable on the"] # [doc = " trait object anyway."] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " trait Foo {"] # [doc = "     type Bar where Self: Sized;"] # [doc = " }"] # [doc = " type Mop = dyn Foo<Bar = ()>;"] # [doc = " ```"] # [doc = ""] # [doc = " {{produces}}"] # [doc = ""] # [doc = " ### Explanation"] # [doc = ""] # [doc = " Just like methods with `Self: Sized` bounds are unavailable on trait"] # [doc = " objects, associated types can be removed from the trait object."] pub UNUSED_ASSOCIATED_TYPE_BOUNDS , Warn , "detects unused `Foo = Bar` bounds in `dyn Trait<Foo = Bar>`" }
};
}
