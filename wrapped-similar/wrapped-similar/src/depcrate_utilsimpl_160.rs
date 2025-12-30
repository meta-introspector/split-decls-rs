// Generated macro for impl_160 (impl)
macro_rules! Depcrate_utilsimpl_160 {
() => {
// Module: crate::utils
// Provides: {"impl_160"}
// Dependencies: {}
impl < 'x , T : DiffableStr + ? Sized > SliceRemapper < 'x , T > { fn new (source : & 'x T , slices : & [& 'x T]) -> SliceRemapper < 'x , T > { let indexes = slices . iter () . scan (0 , | state , item | { let start = * state ; let end = start + item . len () ; * state = end ; Some (start .. end) }) . collect () ; SliceRemapper { source , indexes } } fn slice (& self , range : Range < usize >) -> Option < & 'x T > { let start = self . indexes . get (range . start) ? . start ; let end = self . indexes . get (range . end - 1) ? . end ; Some (self . source . slice (start .. end)) } }
};
}
