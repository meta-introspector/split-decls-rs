// Generated macro for SizableRc (struct)
macro_rules! Depcrate_ptr_rcSizableRc {
() => {
// Module: crate::ptr::rc
// Provides: {"SizableRc"}
// Dependencies: {}
# [doc = " A wrapper around [`Rc`] to implement [`TypeSize`] by allowing you to decide if to count the inner T's size."] # [doc = ""] # [doc = " # Examples"] # [doc = " ```"] # [doc = " # use std::{cell::Cell, rc::Rc};"] # [doc = " # use typesize::{TypeSize, ptr::{SizableRc, Owned, Borrowed}};"] # [doc = " #"] # [doc = " let rc = Rc::new(0);"] # [doc = " let rc_borrow: SizableRc<u8, Borrowed> = rc.clone().into();"] # [doc = " let rc_owner: SizableRc<u8, Owned> = rc.into();"] # [doc = ""] # [doc = " // Just counts the pointer to the internal `RcBox`."] # [doc = " assert_eq!(rc_borrow.get_size(), 0_usize.get_size());"] # [doc = " // Counts the pointer to the `RcBox`, plus the two Cells, and the value."] # [doc = " assert_eq!(rc_owner.get_size(), 0_usize.get_size() + (core::mem::size_of::<usize>() * 2) + 0_u8.get_size());"] # [doc = " ```"] pub struct SizableRc < T , SC : ShouldCountInner > (pub Rc < T > , PhantomData < SC >) ;
};
}
