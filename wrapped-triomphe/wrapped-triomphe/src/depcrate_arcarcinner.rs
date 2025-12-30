// Generated macro for ArcInner (struct)
macro_rules! Depcrate_arcArcInner {
() => {
// Module: crate::arc
// Provides: {"ArcInner"}
// Dependencies: {}
# [doc = " The object allocated by an `Arc<T>`"] # [repr (C)] pub (crate) struct ArcInner < T : ? Sized > { pub (crate) count : atomic :: AtomicUsize , pub (crate) data : T , }
};
}
