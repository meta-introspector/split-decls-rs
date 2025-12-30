// Generated macro for ArcUnion (struct)
macro_rules! Depcrate_arc_unionArcUnion {
() => {
// Module: crate::arc_union
// Provides: {"ArcUnion"}
// Dependencies: {}
# [doc = " A tagged union that can represent `Arc<A>` or `Arc<B>` while only consuming a"] # [doc = " single word. The type is also `NonNull`, and thus can be stored in an Option"] # [doc = " without increasing size."] # [doc = ""] # [doc = " This is functionally equivalent to"] # [doc = " `enum ArcUnion<A, B> { First(Arc<A>), Second(Arc<B>)` but only takes up"] # [doc = " up a single word of stack space."] # [doc = ""] # [doc = " This could probably be extended to support four types if necessary."] pub struct ArcUnion < A , B > { p : ptr :: NonNull < () > , phantom_a : PhantomData < A > , phantom_b : PhantomData < B > , }
};
}
