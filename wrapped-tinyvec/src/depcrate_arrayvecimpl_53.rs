// Generated macro for impl_53 (impl)
macro_rules! Depcrate_arrayvecimpl_53 {
() => {
// Module: crate::arrayvec
// Provides: {"impl_53"}
// Dependencies: {}
impl < A : Array > Iterator for ArrayVecIterator < A > { type Item = A :: Item ; # [inline] fn next (& mut self) -> Option < Self :: Item > { let slice = & mut self . data . as_slice_mut () [self . base as usize .. self . tail as usize] ; let itemref = slice . first_mut () ? ; self . base += 1 ; return Some (core :: mem :: take (itemref)) ; } # [inline (always)] fn size_hint (& self) -> (usize , Option < usize >) { let s = self . tail - self . base ; let s = s as usize ; (s , Some (s)) } # [inline (always)] fn count (self) -> usize { self . size_hint () . 0 } # [inline] fn last (mut self) -> Option < Self :: Item > { self . next_back () } # [inline] fn nth (& mut self , n : usize) -> Option < A :: Item > { let slice = & mut self . data . as_slice_mut () ; let slice = & mut slice [self . base as usize .. self . tail as usize] ; if let Some (x) = slice . get_mut (n) { self . base += n as u16 + 1 ; return Some (core :: mem :: take (x)) ; } self . base = self . tail ; return None ; } }
};
}
