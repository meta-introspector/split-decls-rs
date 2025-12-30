// Generated macro for impl_108 (impl)
macro_rules! Depcrateimpl_108 {
() => {
// Module: crate
// Provides: {"impl_108"}
// Dependencies: {}
# [doc = " Private helper methods for `Splice::drop`"] impl < T > Drain < '_ , T > { # [doc = " The range from `self.vec.len` to `self.tail_start` contains elements"] # [doc = " that have been moved out."] # [doc = " Fill that range as much as possible with new elements from the `replace_with` iterator."] # [doc = " Returns `true` if we filled the entire range. (`replace_with.next()` didn’t return `None`.)"] unsafe fn fill < I : Iterator < Item = T > > (& mut self , replace_with : & mut I) -> bool { let vec = unsafe { self . vec . as_mut () } ; let range_start = vec . len () ; let range_end = self . end ; let range_slice = unsafe { slice :: from_raw_parts_mut (vec . data_raw () . add (range_start) , range_end - range_start) } ; for place in range_slice { if let Some (new_item) = replace_with . next () { unsafe { ptr :: write (place , new_item) } ; vec . set_len (vec . len () + 1) ; } else { return false ; } } true } # [doc = " Makes room for inserting more elements before the tail."] unsafe fn move_tail (& mut self , additional : usize) { let vec = unsafe { self . vec . as_mut () } ; let len = self . end + self . tail ; vec . reserve (len . checked_add (additional) . unwrap_cap_overflow ()) ; let new_tail_start = self . end + additional ; unsafe { let src = vec . data_raw () . add (self . end) ; let dst = vec . data_raw () . add (new_tail_start) ; ptr :: copy (src , dst , self . tail) ; } self . end = new_tail_start ; } }
};
}
