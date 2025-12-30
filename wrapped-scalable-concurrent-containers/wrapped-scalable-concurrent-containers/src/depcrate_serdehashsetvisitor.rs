// Generated macro for HashSetVisitor (struct)
macro_rules! Depcrate_serdeHashSetVisitor {
() => {
// Module: crate::serde
// Provides: {"HashSetVisitor"}
// Dependencies: {}
# [doc = " Helper type to allow `serde` to access [`HashSet`] entries."] pub struct HashSetVisitor < K : Eq + Hash , H : BuildHasher > { marker : PhantomData < fn () -> HashSet < K , H > > , }
};
}
