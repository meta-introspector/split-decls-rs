// Generated macro for CloneableCart (trait)
macro_rules! Depcrate_yokeCloneableCart {
() => {
// Module: crate::yoke
// Provides: {"CloneableCart"}
// Dependencies: {}
# [doc = " This trait marks cart types that do not change source on cloning"] # [doc = ""] # [doc = " This is conceptually similar to [`stable_deref_trait::CloneStableDeref`],"] # [doc = " however [`stable_deref_trait::CloneStableDeref`] is not (and should not) be"] # [doc = " implemented on [`Option`] (since it's not [`Deref`]). [`CloneableCart`] essentially is"] # [doc = " \"if there _is_ data to borrow from here, cloning the cart gives you an additional"] # [doc = " handle to the same data\"."] # [doc = ""] # [doc = " # Safety"] # [doc = " This trait is safe to implement on `StableDeref` types which, once `Clone`d, point to the same underlying data and retain ownership."] # [doc = ""] # [doc = " This trait can also be implemented on aggregates of such types like `Option<T: CloneableCart>` and `(T: CloneableCart, U: CloneableCart)`."] # [doc = ""] # [doc = " Essentially, all data that could be referenced by a Yokeable (i.e. data that is referenced via a StableDeref) must retain the same"] # [doc = " pointer and ownership semantics once cloned."] pub unsafe trait CloneableCart : Clone { }
};
}
