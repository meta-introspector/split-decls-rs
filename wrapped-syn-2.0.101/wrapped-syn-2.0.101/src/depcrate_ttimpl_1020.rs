// Generated macro for impl_1020 (impl)
macro_rules! Depcrate_ttimpl_1020 {
() => {
// Module: crate::tt
// Provides: {"impl_1020"}
// Dependencies: {}
impl < 'a > Hash for TokenStreamHelper < 'a > { fn hash < H : Hasher > (& self , state : & mut H) { let tts = self . 0 . clone () . into_iter () . collect :: < Vec < _ > > () ; tts . len () . hash (state) ; for tt in tts { TokenTreeHelper (& tt) . hash (state) ; } } }
};
}
