// Generated macro for Shared (struct)
macro_rules! Depcrate_pageShared {
() => {
// Module: crate::page
// Provides: {"Shared"}
// Dependencies: {}
pub (crate) struct Shared < T , C > { # [doc = " The remote free list"] # [doc = ""] # [doc = " Slots freed from a remote thread are pushed onto this list."] remote : stack :: TransferStack < C > , size : usize , prev_sz : usize , slab : UnsafeCell < Option < Slots < T , C > > > , }
};
}
