// Generated macro for impl_369 (impl)
macro_rules! Depcrate_util_wake_listimpl_369 {
() => {
// Module: crate::util::wake_list
// Provides: {"impl_369"}
// Dependencies: {}
impl WakeList { pub (crate) fn new () -> Self { const UNINIT_WAKER : MaybeUninit < Waker > = MaybeUninit :: uninit () ; Self { inner : [UNINIT_WAKER ; NUM_WAKERS] , curr : 0 , } } # [inline] pub (crate) fn can_push (& self) -> bool { self . curr < NUM_WAKERS } pub (crate) fn push (& mut self , val : Waker) { debug_assert ! (self . can_push ()) ; self . inner [self . curr] = MaybeUninit :: new (val) ; self . curr += 1 ; } pub (crate) fn wake_all (& mut self) { struct DropGuard { start : * mut Waker , end : * mut Waker , } impl Drop for DropGuard { fn drop (& mut self) { let len = unsafe { self . end . offset_from (self . start) } as usize ; let slice = ptr :: slice_from_raw_parts_mut (self . start , len) ; unsafe { ptr :: drop_in_place (slice) } ; } } debug_assert ! (self . curr <= NUM_WAKERS) ; let mut guard = { let start = self . inner . as_mut_ptr () . cast :: < Waker > () ; let end = unsafe { start . add (self . curr) } ; self . curr = 0 ; DropGuard { start , end } } ; while ! ptr :: eq (guard . start , guard . end) { let waker = unsafe { ptr :: read (guard . start) } ; guard . start = unsafe { guard . start . add (1) } ; waker . wake () ; } } }
};
}
