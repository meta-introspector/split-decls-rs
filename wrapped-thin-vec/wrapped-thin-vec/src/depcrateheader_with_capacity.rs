// Generated macro for header_with_capacity (function)
macro_rules! Depcrateheader_with_capacity {
() => {
// Module: crate
// Provides: {"header_with_capacity"}
// Dependencies: {}
# [doc = " Allocates a header (and array) for a `ThinVec<T>` with the given capacity."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if the required size overflows `isize::MAX`."] fn header_with_capacity < T > (cap : usize , is_auto : bool) -> NonNull < Header > { debug_assert ! (cap > 0) ; unsafe { let layout = layout :: < T > (cap) ; let header = alloc (layout) as * mut Header ; if header . is_null () { handle_alloc_error (layout) } ptr :: write (header , Header { _len : 0 , _cap : if mem :: size_of :: < T > () == 0 { MAX_CAP as SizeType } else { pack_capacity_and_auto (assert_size (cap) , is_auto) } , } ,) ; NonNull :: new_unchecked (header) } }
};
}
