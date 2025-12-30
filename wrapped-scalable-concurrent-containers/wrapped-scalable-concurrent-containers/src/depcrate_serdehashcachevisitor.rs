// Generated macro for HashCacheVisitor (struct)
macro_rules! Depcrate_serdeHashCacheVisitor {
() => {
// Module: crate::serde
// Provides: {"HashCacheVisitor"}
// Dependencies: {}
# [doc = " Helper type to allow `serde` to access [`HashCache`] entries."] pub struct HashCacheVisitor < K : Eq + Hash , V , H : BuildHasher > { # [allow (clippy :: type_complexity)] marker : PhantomData < fn () -> HashCache < K , V , H > > , }
};
}
