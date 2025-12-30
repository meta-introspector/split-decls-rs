// Generated macro for impl_211 (impl)
macro_rules! Depcrate_unique_arcimpl_211 {
() => {
// Module: crate::unique_arc
// Provides: {"impl_211"}
// Dependencies: {}
impl < T > UniqueArc < T > { # [inline] # [doc = " Construct a new UniqueArc"] pub fn new (data : T) -> Self { UniqueArc (Arc :: new (data)) } # [doc = " Construct an uninitialized arc"] # [inline] pub fn new_uninit () -> UniqueArc < MaybeUninit < T > > { unsafe { let layout = Layout :: new :: < ArcInner < MaybeUninit < T > > > () ; let ptr = alloc :: alloc :: alloc (layout) ; let mut p = NonNull :: new (ptr) . unwrap_or_else (| | alloc :: alloc :: handle_alloc_error (layout)) . cast :: < ArcInner < MaybeUninit < T > > > () ; ptr :: write (& mut p . as_mut () . count , AtomicUsize :: new (1)) ; UniqueArc (Arc { p , phantom : PhantomData , }) } } # [doc = " Gets the inner value of the unique arc"] pub fn into_inner (this : Self) -> T { let this = ManuallyDrop :: new (this . 0) ; debug_assert ! (this . is_unique () , "attempted to call `.into_inner()` on a `UniqueArc` with a non-zero ref count" ,) ; unsafe { Box :: from_raw (this . ptr ()) . data } } }
};
}
