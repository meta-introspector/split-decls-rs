// Generated macro for impl_95 (impl)
macro_rules! Depcrate_linked_listimpl_95 {
() => {
// Module: crate::linked_list
// Provides: {"impl_95"}
// Dependencies: {}
impl < T > LinkedEntry < T > { # [doc = " Extracts the inner instance of `T`."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " This method has to be called at most once per [`LinkedEntry`], and the caller needs to make"] # [doc = " sure that the [`LinkedEntry`] is not accessed via [`LinkedList`] methods."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use sdd::Stack;"] # [doc = ""] # [doc = " let stack: Stack<usize> = Stack::default();"] # [doc = ""] # [doc = " stack.push(37);"] # [doc = ""] # [doc = " let mut entry = stack.pop().unwrap();"] # [doc = " let pushed = unsafe { entry.get_mut().unwrap().take_inner() };"] # [doc = " assert_eq!(pushed, 37);"] # [doc = " ```"] # [inline] pub unsafe fn take_inner (& mut self) -> T { unsafe { self . instance . take () . unwrap_unchecked () } } # [inline] pub (super) fn new (val : T) -> Self { Self { instance : Some (val) , next : AtomicShared :: default () , } } # [doc = " Returns a reference to `next`."] # [inline] pub (super) fn next (& self) -> & AtomicShared < Self > { & self . next } }
};
}
