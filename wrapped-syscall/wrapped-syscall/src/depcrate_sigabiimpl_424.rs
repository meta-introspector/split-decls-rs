// Generated macro for impl_424 (impl)
macro_rules! Depcrate_sigabiimpl_424 {
() => {
// Module: crate::sigabi
// Provides: {"impl_424"}
// Dependencies: {}
impl Sigcontrol { pub fn currently_pending_unblocked (& self , proc : & SigProcControl) -> u64 { let proc_pending = proc . pending . load (Ordering :: Relaxed) ; let [w0 , w1] = core :: array :: from_fn (| i | { let w = self . word [i] . load (Ordering :: Relaxed) ; ((w | (proc_pending >> (i * 32))) & 0xffff_ffff) & (w >> 32) }) ; w0 | (w1 << 32) } pub fn set_allowset (& self , new_allowset : u64) -> u64 { let [w0 , w1] = self . word . each_ref () . map (| w | w . load (Ordering :: Relaxed)) ; let old_a0 = w0 & 0xffff_ffff_0000_0000 ; let old_a1 = w1 & 0xffff_ffff_0000_0000 ; let new_a0 = (new_allowset & 0xffff_ffff) << 32 ; let new_a1 = new_allowset & 0xffff_ffff_0000_0000 ; let prev_w0 = self . word [0] . fetch_add (new_a0 . wrapping_sub (old_a0) , Ordering :: Relaxed) ; let prev_w1 = self . word [0] . fetch_add (new_a1 . wrapping_sub (old_a1) , Ordering :: Relaxed) ; let up0 = prev_w0 & (prev_w0 >> 32) ; let up1 = prev_w1 & (prev_w1 >> 32) ; up0 | (up1 << 32) } }
};
}
