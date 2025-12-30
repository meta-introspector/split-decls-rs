// Generated macro for SizableArc (struct)
macro_rules! Depcrate_ptr_arcSizableArc {
() => {
// Module: crate::ptr::arc
// Provides: {"SizableArc"}
// Dependencies: {}
# [doc = " A wrapper around [`Arc`] to implement [`TypeSize`] by allowing you to decide if to count the inner T's size."] # [doc = ""] # [doc = " # Examples"] # [doc = " ```"] # [doc = " # use std::{cell::Cell, sync::Arc};"] # [doc = " # use typesize::{TypeSize, ptr::{SizableArc, Owned, Borrowed}};"] # [doc = " #"] # [doc = " let arc = Arc::new(0);"] # [doc = " let arc_borrow: SizableArc<u8, Borrowed> = arc.clone().into();"] # [doc = " let arc_owner: SizableArc<u8, Owned> = arc.into();"] # [doc = ""] # [doc = " // Just counts the pointer to the internal `ArcBox`."] # [doc = " assert_eq!(arc_borrow.get_size(), 0_usize.get_size());"] # [doc = " // Counts the pointer to the `ArcBox`, plus the two AtomicUsize, and the value."] # [doc = " assert_eq!(arc_owner.get_size(), 0_usize.get_size() + (core::mem::size_of::<usize>() * 2) + 0_u8.get_size());"] # [doc = " ```"] pub struct SizableArc < T , SC : ShouldCountInner > (pub Arc < T > , PhantomData < SC >) ;
};
}
