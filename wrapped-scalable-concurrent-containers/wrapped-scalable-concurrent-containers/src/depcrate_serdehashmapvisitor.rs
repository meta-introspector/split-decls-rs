// Generated macro for HashMapVisitor (struct)
macro_rules! Depcrate_serdeHashMapVisitor {
() => {
// Module: crate::serde
// Provides: {"HashMapVisitor"}
// Dependencies: {}
# [doc = " Helper type to allow `serde` to access [`HashMap`] entries."] pub struct HashMapVisitor < K : Eq + Hash , V , H : BuildHasher > { # [allow (clippy :: type_complexity)] marker : PhantomData < fn () -> HashMap < K , V , H > > , }
};
}
