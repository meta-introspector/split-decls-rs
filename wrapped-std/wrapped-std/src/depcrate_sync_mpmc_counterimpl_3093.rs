// Generated macro for impl_3093 (impl)
macro_rules! Depcrate_sync_mpmc_counterimpl_3093 {
() => {
// Module: crate::sync::mpmc::counter
// Provides: {"impl_3093"}
// Dependencies: {}
impl < C > Receiver < C > { # [doc = " Returns the internal `Counter`."] fn counter (& self) -> & Counter < C > { unsafe { & * self . counter } } # [doc = " Acquires another receiver reference."] pub (crate) fn acquire (& self) -> Receiver < C > { let count = self . counter () . receivers . fetch_add (1 , Ordering :: Relaxed) ; if count > isize :: MAX as usize { process :: abort () ; } Receiver { counter : self . counter } } # [doc = " Releases the receiver reference."] # [doc = ""] # [doc = " Function `disconnect` will be called if this is the last receiver reference."] pub (crate) unsafe fn release < F : FnOnce (& C) -> bool > (& self , disconnect : F) { if self . counter () . receivers . fetch_sub (1 , Ordering :: AcqRel) == 1 { disconnect (& self . counter () . chan) ; if self . counter () . destroy . swap (true , Ordering :: AcqRel) { drop (unsafe { Box :: from_raw (self . counter) }) ; } } } }
};
}
