macro_rules! deps {
    () => {
        PointerMetadata!();
        KnownLayout!();
        AllocError!();
    };
}

macro_rules! new_box {
    () => {
        deps!();
        # [doc = " Uses `allocate` to create a `Box<T>`."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " Returns an error on allocation failure. Allocation failure is guaranteed"] # [doc = " never to cause a panic or an abort."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " `allocate` must be either `alloc::alloc::alloc` or"] # [doc = " `alloc::alloc::alloc_zeroed`. The referent of the box returned by `new_box`"] # [doc = " has the same bit-validity as the referent of the pointer returned by the"] # [doc = " given `allocate` and sufficient size to store `T` with `meta`."] # [must_use = "has no side effects (other than allocation)"] # [cfg (feature = "alloc")] # [inline] pub (crate) unsafe fn new_box < T > (meta : T :: PointerMetadata , allocate : unsafe fn (core :: alloc :: Layout) -> * mut u8 ,) -> Result < alloc :: boxed :: Box < T > , AllocError > where T : ? Sized + crate :: KnownLayout , { let size = match T :: size_for_metadata (meta) { Some (size) => size , None => return Err (AllocError) , } ; let align = T :: LAYOUT . align . get () ; # [allow (clippy :: as_conversions)] let max_alloc = (isize :: MAX as usize) . saturating_sub (align) ; if size > max_alloc { return Err (AllocError) ; } let layout = Layout :: from_size_align (size , align) . or (Err (AllocError)) ? ; let ptr = if layout . size () != 0 { let ptr = unsafe { allocate (layout) } ; match NonNull :: new (ptr) { Some (ptr) => ptr , None => return Err (AllocError) , } } else { let align = T :: LAYOUT . align . get () ; # [allow (unknown_lints)] # [allow (clippy :: useless_transmute , integer_to_ptr_transmutes)] let dangling = unsafe { mem :: transmute :: < usize , * mut u8 > (align) } ; unsafe { NonNull :: new_unchecked (dangling) } } ; let ptr = T :: raw_from_ptr_len (ptr , meta) ; # [allow (clippy :: undocumented_unsafe_blocks)] Ok (unsafe { alloc :: boxed :: Box :: from_raw (ptr . as_ptr ()) }) }
    };
}

new_box!()