// Generated macro for VariantIterator (trait)
macro_rules! DepcrateVariantIterator {
() => {
// Module: crate
// Provides: {"VariantIterator"}
// Dependencies: {}
pub trait VariantIterator : Sized { type Iterator : Iterator < Item = Self > ; fn iter () -> Self :: Iterator ; }
};
}
