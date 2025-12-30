// Generated macro for impl_32 (impl)
macro_rules! Depcrate_arcimpl_32 {
() => {
// Module: crate::arc
// Provides: {"impl_32"}
// Dependencies: {}
impl < T : ? Sized > ArcInner < T > { # [doc = " Compute the offset of the `data` field within `ArcInner<T>`."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " - The pointer must be created from `Arc::into_raw` or similar functions"] # [doc = " - The pointee must be initialized (`&*value` must not be UB)."] # [doc = "   That happens automatically if the pointer comes from `Arc` and type was not changed."] # [doc = "   This is **not** the case, for example, when `Arc` was uninitialized `MaybeUninit<T>`"] # [doc = "   and the pointer was cast to `*const T`."] unsafe fn offset_of_data (value : * const T) -> usize { let value = & * value ; let layout = Layout :: new :: < atomic :: AtomicUsize > () ; let (_ , offset) = layout . extend (Layout :: for_value (value)) . unwrap () ; offset } }
};
}
