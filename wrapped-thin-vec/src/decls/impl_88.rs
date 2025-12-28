macro_rules! deps {
    () => {
        AutoBuffer!();
        Header!();
        ThinVec!();
        AutoThinVec!();
    };
}

macro_rules! impl_88 {
    () => {
        deps!();
        # [cfg (feature = "gecko-ffi")] impl < T , const N : usize > AutoThinVec < T , N > { # [doc = " Implementation detail for the auto_thin_vec macro."] # [inline] # [doc (hidden)] pub fn new_unpinned () -> Self { assert ! (std :: mem :: align_of ::< T > () <= 8 , "Can't handle alignments greater than 8") ; assert_eq ! (std :: mem :: offset_of ! (Self , buffer) , AUTO_ARRAY_HEADER_OFFSET) ; Self { inner : ThinVec :: new () , buffer : AutoBuffer { header : Header { _len : 0 , _cap : pack_capacity_and_auto (N as SizeType , true) , } , buffer : mem :: MaybeUninit :: uninit () , } , _pinned : std :: marker :: PhantomPinned , } } # [doc = " Returns a raw pointer to the inner ThinVec. Note that if you dereference it from rust, you"] # [doc = " need to make sure not to move the ThinVec manually via something like"] # [doc = " `std::mem::take(&mut auto_vec)`."] pub fn as_mut_ptr (self : std :: pin :: Pin < & mut Self >) -> * mut ThinVec < T > { debug_assert ! (self . is_auto_array ()) ; unsafe { & mut self . get_unchecked_mut () . inner } } # [inline] pub unsafe fn shrink_to_fit_known_singleton (self : std :: pin :: Pin < & mut Self >) { debug_assert ! (self . is_singleton ()) ; let this = unsafe { self . get_unchecked_mut () } ; this . buffer . header . set_len (0) ; this . inner . ptr = NonNull :: new_unchecked (& mut this . buffer . header) ; debug_assert ! (this . inner . is_auto_array ()) ; debug_assert ! (this . inner . uses_stack_allocated_buffer ()) ; } pub fn shrink_to_fit (self : std :: pin :: Pin < & mut Self >) { let this = unsafe { self . get_unchecked_mut () } ; this . inner . shrink_to_fit () ; debug_assert ! (this . inner . is_auto_array ()) ; } }
    };
}

impl_88!()