// Generated macro for impl_131 (impl)
macro_rules! Depcrate_headerimpl_131 {
() => {
// Module: crate::header
// Provides: {"impl_131"}
// Dependencies: {}
impl < T > From < Box < T > > for Arc < T > { fn from (b : Box < T >) -> Self { let layout = Layout :: for_value :: < T > (& b) ; let inner = unsafe { Self :: allocate_for_layout (layout , | mem | mem as * mut ArcInner < T >) } ; unsafe { let src = Box :: into_raw (b) ; let dst = addr_of_mut ! ((* inner . as_ptr ()) . data) ; ptr :: copy_nonoverlapping (src , dst , 1) ; drop (Box :: < ManuallyDrop < T > > :: from_raw (src as _)) ; } Arc { p : inner , phantom : PhantomData , } } }
};
}
