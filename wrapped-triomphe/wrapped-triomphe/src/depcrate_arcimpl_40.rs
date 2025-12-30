// Generated macro for impl_40 (impl)
macro_rules! Depcrate_arcimpl_40 {
() => {
// Module: crate::arc
// Provides: {"impl_40"}
// Dependencies: {}
impl < H , T > Arc < HeaderSlice < H , [T] > > { pub (super) fn allocate_for_header_and_slice (len : usize ,) -> NonNull < ArcInner < HeaderSlice < H , [T] > > > { let layout = Layout :: new :: < H > () . extend (Layout :: array :: < T > (len) . unwrap ()) . unwrap () . 0 . pad_to_align () ; unsafe { Arc :: allocate_for_layout (layout , | mem | { let fake_slice = ptr :: slice_from_raw_parts_mut (mem as * mut T , len) ; fake_slice as * mut ArcInner < HeaderSlice < H , [T] > > }) } } }
};
}
