// Generated macro for impl_17 (impl)
macro_rules! Depcrate_from_iterimpl_17 {
() => {
// Module: crate::from_iter
// Provides: {"impl_17"}
// Dependencies: {}
impl < T , const N : usize > FromIterator < T > for ArrayFromIter < T , N > { fn from_iter < I : IntoIterator < Item = T > > (iter : I) -> Self { let mut buffer = uninit_buf :: < T , N > () ; let mut iter = iter . into_iter () ; let mut buf_iter = buffer . iter_mut () ; let mut num_written = 0 ; loop { let item = iter . next () ; let slot = buf_iter . next () ; match (item , slot) { (Some (item) , Some (slot)) => { num_written += 1 ; slot . write (item) ; } (Some (_) , None) | (None , Some (_)) => { buffer . iter_mut () . take (num_written) . for_each (| slot | unsafe { slot . assume_init_drop () }) ; return Self (None) ; } (None , None) => return Self (Some (unsafe { mark_initialized (buffer) })) , } ; } } }
};
}
