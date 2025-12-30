// Generated macro for IntoEnumIterator (trait)
macro_rules! DepcrateIntoEnumIterator {
() => {
// Module: crate
// Provides: {"IntoEnumIterator"}
// Dependencies: {}
# [doc = " This trait designates that an `Enum` can be iterated over. It can"] # [doc = " be auto generated using the [`EnumIter`](derive.EnumIter.html) derive macro."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " # use std::fmt::Debug;"] # [doc = " // You need to bring the type into scope to use it!!!"] # [doc = " use strum::{EnumIter, IntoEnumIterator};"] # [doc = ""] # [doc = " #[derive(EnumIter, Debug)]"] # [doc = " enum Color {"] # [doc = "     Red,"] # [doc = "     Green { range: usize },"] # [doc = "     Blue(usize),"] # [doc = "     Yellow,"] # [doc = " }"] # [doc = ""] # [doc = " // Iterate over the items in an enum and perform some function on them."] # [doc = " fn generic_iterator<E, F>(pred: F)"] # [doc = " where"] # [doc = "     E: IntoEnumIterator,"] # [doc = "     F: Fn(E),"] # [doc = " {"] # [doc = "     for e in E::iter() {"] # [doc = "         pred(e)"] # [doc = "     }"] # [doc = " }"] # [doc = ""] # [doc = " generic_iterator::<Color, _>(|color| println!(\"{:?}\", color));"] # [doc = " ```"] pub trait IntoEnumIterator : Sized { type Iterator : Iterator < Item = Self > + Clone + DoubleEndedIterator + ExactSizeIterator + FusedIterator ; fn iter () -> Self :: Iterator ; }
};
}
