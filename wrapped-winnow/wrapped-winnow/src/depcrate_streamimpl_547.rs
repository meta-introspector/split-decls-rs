// Generated macro for impl_547 (impl)
macro_rules! Depcrate_streamimpl_547 {
() => {
// Module: crate::stream
// Provides: {"impl_547"}
// Dependencies: {}
impl < T > Offset for & [T] { # [inline] fn offset_from (& self , start : & Self) -> usize { let fst = (* start) . as_ptr () ; let snd = (* self) . as_ptr () ; debug_assert ! (fst <= snd , "`Offset::offset_from({snd:?}, {fst:?})` only accepts slices of `self`") ; (snd as usize - fst as usize) / core :: mem :: size_of :: < T > () } }
};
}
