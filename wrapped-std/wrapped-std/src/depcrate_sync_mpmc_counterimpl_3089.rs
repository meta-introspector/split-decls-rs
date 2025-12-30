// Generated macro for impl_3089 (impl)
macro_rules! Depcrate_sync_mpmc_counterimpl_3089 {
() => {
// Module: crate::sync::mpmc::counter
// Provides: {"impl_3089"}
// Dependencies: {}
impl < C > Sender < C > { # [doc = " Returns the internal `Counter`."] fn counter (& self) -> & Counter < C > { unsafe { & * self . counter } } # [doc = " Acquires another sender reference."] pub (crate) fn acquire (& self) -> Sender < C > { let count = self . counter () . senders . fetch_add (1 , Ordering :: Relaxed) ; if count > isize :: MAX as usize { process :: abort () ; } Sender { counter : self . counter } } # [doc = " Releases the sender reference."] # [doc = ""] # [doc = " Function `disconnect` will be called if this is the last sender reference."] pub (crate) unsafe fn release < F : FnOnce (& C) -> bool > (& self , disconnect : F) { if self . counter () . senders . fetch_sub (1 , Ordering :: AcqRel) == 1 { disconnect (& self . counter () . chan) ; if self . counter () . destroy . swap (true , Ordering :: AcqRel) { drop (unsafe { Box :: from_raw (self . counter) }) ; } } } }
};
}
