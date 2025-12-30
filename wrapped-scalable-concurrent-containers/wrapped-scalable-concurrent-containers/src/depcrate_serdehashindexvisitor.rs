// Generated macro for HashIndexVisitor (struct)
macro_rules! Depcrate_serdeHashIndexVisitor {
() => {
// Module: crate::serde
// Provides: {"HashIndexVisitor"}
// Dependencies: {}
# [doc = " Helper type to allow `serde` to access [`HashIndex`] entries."] pub struct HashIndexVisitor < K : Eq + Hash , V , H : BuildHasher > { # [allow (clippy :: type_complexity)] marker : PhantomData < fn () -> HashIndex < K , V , H > > , }
};
}
