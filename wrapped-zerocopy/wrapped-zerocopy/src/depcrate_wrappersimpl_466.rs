// Generated macro for impl_466 (impl)
macro_rules! Depcrate_wrappersimpl_466 {
() => {
// Module: crate::wrappers
// Provides: {"impl_466"}
// Dependencies: {}
# [doc (hidden)] impl < T : ? Sized + KnownLayout > MaybeUninit < T > { # [doc = " Constructs a `MaybeUninit<T>` initialized with the given value."] # [inline (always)] pub fn new (val : T) -> Self where T : Sized , Self : Sized , { unsafe { crate :: util :: transmute_unchecked (val) } } # [doc = " Constructs an uninitialized `MaybeUninit<T>`."] # [must_use] # [inline (always)] pub fn uninit () -> Self where T : Sized , Self : Sized , { let uninit = CoreMaybeUninit :: < T > :: uninit () ; unsafe { crate :: util :: transmute_unchecked (uninit) } } # [doc = " Creates a `Box<MaybeUninit<T>>`."] # [doc = ""] # [doc = " This function is useful for allocating large, uninit values on the heap"] # [doc = " without ever creating a temporary instance of `Self` on the stack."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " Returns an error on allocation failure. Allocation failure is guaranteed"] # [doc = " never to cause a panic or an abort."] # [cfg (feature = "alloc")] # [inline] pub fn new_boxed_uninit (meta : T :: PointerMetadata) -> Result < Box < Self > , AllocError > { unsafe { crate :: util :: new_box (meta , alloc :: alloc :: alloc) } } # [doc = " Extracts the value from the `MaybeUninit<T>` container."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The caller must ensure that `self` is in an bit-valid state. Depending"] # [doc = " on subsequent use, it may also need to be in a library-valid state."] # [inline (always)] pub unsafe fn assume_init (self) -> T where T : Sized , Self : Sized , { unsafe { crate :: util :: transmute_unchecked (self) } } }
};
}
