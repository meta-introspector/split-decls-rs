// Generated macro for impl_340 (impl)
macro_rules! Depcrate_varzerovec_componentsimpl_340 {
() => {
// Module: crate::varzerovec::components
// Provides: {"impl_340"}
// Dependencies: {}
impl < 'a , T , F > VarZeroVecComponents < 'a , T , F > where T : VarULE , T : ? Sized , F : VarZeroVecFormat , { # [doc = " Binary searches a sorted `VarZeroVecComponents<T>` for the given predicate. For more information, see"] # [doc = " the primitive function [`binary_search_by`](slice::binary_search_by)."] pub fn binary_search_by (& self , predicate : impl FnMut (& T) -> Ordering) -> Result < usize , usize > { unsafe { self . binary_search_in_range_unchecked (predicate , 0 .. self . len ()) } } pub fn binary_search_in_range_by (& self , predicate : impl FnMut (& T) -> Ordering , range : Range < usize > ,) -> Option < Result < usize , usize > > { if range . end > self . len () { return None ; } if range . end < range . start { return None ; } let range_absolute = unsafe { self . binary_search_in_range_unchecked (predicate , range . clone ()) } ; Some (range_absolute . map (| o | o - range . start) . map_err (| e | e - range . start) ,) } # [doc = " Safety: range must be in range for the slice (start <= len, end <= len, start <= end)"] unsafe fn binary_search_in_range_unchecked (& self , mut predicate : impl FnMut (& T) -> Ordering , range : Range < usize > ,) -> Result < usize , usize > { let mut start = range . start ; let mut end = range . end ; let mut size ; while start < end { size = end - start ; let mid = start + size / 2 ; let cmp = predicate (self . get_unchecked (mid)) ; match cmp { Ordering :: Less => { start = mid + 1 ; } Ordering :: Greater => { end = mid ; } Ordering :: Equal => return Ok (mid) , } } Err (start) } }
};
}
