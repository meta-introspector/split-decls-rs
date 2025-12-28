macro_rules! check_raw_pointer {
    () => {
        # [doc = " Convert a `*mut c_void` to a `*mut T`, checking that it is not null,"] # [doc = " misaligned, or pointing to a region of memory that wraps around the address"] # [doc = " space."] pub (crate) fn check_raw_pointer < T > (value : * mut c_void) -> Option < NonNull < T > > { if (value as usize) . checked_add (size_of :: < T > ()) . is_none () || (value as usize) % align_of :: < T > () != 0 { return None ; } NonNull :: new (value . cast ()) }
    };
}

check_raw_pointer!();