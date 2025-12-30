// Generated macro for NoBoundsException (enum)
macro_rules! Depcrate_unusedNoBoundsException {
() => {
// Module: crate::unused
// Provides: {"NoBoundsException"}
// Dependencies: {}
# [doc = " Whether parentheses may be omitted from a type without resulting in ambiguity."] # [doc = ""] # [doc = " ```"] # [doc = " type Example = Box<dyn Fn() -> &'static (dyn Send) + Sync>;"] # [doc = " ```"] # [doc = ""] # [doc = " Here, `&'static (dyn Send) + Sync` is a `TypeNoBounds`. As such, it may not directly"] # [doc = " contain `ImplTraitType` or `TraitObjectType` which is why `(dyn Send)` is parenthesized."] # [doc = " However, an exception is made for `ImplTraitTypeOneBound` and `TraitObjectTypeOneBound`."] # [doc = " The following is accepted because there is no `+`."] # [doc = ""] # [doc = " ```"] # [doc = " type Example = Box<dyn Fn() -> &'static dyn Send>;"] # [doc = " ```"] enum NoBoundsException { # [doc = " The type must be parenthesized."] None , # [doc = " The type is the last bound of the containing type expression. If it has exactly one bound,"] # [doc = " parentheses around the type are unnecessary."] OneBound , }
};
}
