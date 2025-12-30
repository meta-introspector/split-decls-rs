// Generated macro for thin_to_thick (function)
macro_rules! Depcrate_thin_arcthin_to_thick {
() => {
// Module: crate::thin_arc
// Provides: {"thin_to_thick"}
// Dependencies: {}
# [inline] fn thin_to_thick < H , T > (arc : & ThinArc < H , T >) -> * mut ArcInner < HeaderSliceWithLengthProtected < H , T > > { let thin = arc . ptr . as_ptr () ; let len = unsafe { (* thin) . data . header . length } ; let fake_slice = ptr :: slice_from_raw_parts_mut (thin as * mut T , len) ; fake_slice as * mut ArcInner < HeaderSliceWithLengthProtected < H , T > > }
};
}
