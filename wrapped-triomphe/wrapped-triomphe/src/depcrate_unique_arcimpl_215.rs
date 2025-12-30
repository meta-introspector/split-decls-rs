// Generated macro for impl_215 (impl)
macro_rules! Depcrate_unique_arcimpl_215 {
() => {
// Module: crate::unique_arc
// Provides: {"impl_215"}
// Dependencies: {}
impl < H , T > UniqueArc < HeaderSlice < H , [MaybeUninit < T >] > > { # [doc = " Creates an Arc for a HeaderSlice using the given header struct and allocated space"] # [doc = " for an unitialized slice of length `len`."] # [inline] pub fn from_header_and_uninit_slice (header : H , len : usize) -> Self { let inner = Arc :: < HeaderSlice < H , [MaybeUninit < T >] > > :: allocate_for_header_and_slice (len) ; unsafe { let dst = addr_of_mut ! ((* inner . as_ptr ()) . data . header) ; ptr :: write (dst , header) ; } Self (Arc { p : inner , phantom : PhantomData , }) } # [doc = " # Safety"] # [doc = ""] # [doc = " Must initialize all fields before calling this function."] # [inline] pub unsafe fn assume_init_slice_with_header (self) -> UniqueArc < HeaderSlice < H , [T] > > { unsafe { core :: mem :: transmute (self) } } }
};
}
