// Generated macro for CloneStableDeref (trait)
macro_rules! DepcrateCloneStableDeref {
() => {
// Module: crate
// Provides: {"CloneStableDeref"}
// Dependencies: {}
# [doc = "\nAn unsafe marker trait for types where clones deref to the same address. This has all the requirements of StableDeref, and additionally requires that after calling clone(), both the old and new value deref to the same address. For example, Rc and Arc implement CloneStableDeref, but Box and Vec do not.\n\nNote that a single type should never implement both DerefMut and CloneStableDeref. If it did, this would let you get two mutable references to the same location, by cloning and then calling deref_mut() on both values.\n"] pub unsafe trait CloneStableDeref : StableDeref + Clone { }
};
}
