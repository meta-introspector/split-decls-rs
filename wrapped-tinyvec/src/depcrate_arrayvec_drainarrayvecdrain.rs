// Generated macro for ArrayVecDrain (struct)
macro_rules! Depcrate_arrayvec_drainArrayVecDrain {
() => {
// Module: crate::arrayvec_drain
// Provides: {"ArrayVecDrain"}
// Dependencies: {}
# [doc = " Draining iterator for [`ArrayVec`]"] # [doc = ""] # [doc = " See [`ArrayVec::drain`](ArrayVec::drain)"] pub struct ArrayVecDrain < 'a , T : 'a + Default > { iter : slice :: IterMut < 'a , T > , }
};
}
