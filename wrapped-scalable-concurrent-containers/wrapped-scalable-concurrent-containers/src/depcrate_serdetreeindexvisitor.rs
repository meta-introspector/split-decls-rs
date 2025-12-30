// Generated macro for TreeIndexVisitor (struct)
macro_rules! Depcrate_serdeTreeIndexVisitor {
() => {
// Module: crate::serde
// Provides: {"TreeIndexVisitor"}
// Dependencies: {}
# [doc = " Helper type to allow `serde` to access [`TreeIndex`] entries."] pub struct TreeIndexVisitor < K : 'static + Clone + Ord , V : 'static + Clone > { # [allow (clippy :: type_complexity)] marker : PhantomData < fn () -> TreeIndex < K , V > > , }
};
}
