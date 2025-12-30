// Generated macro for impl_1693 (impl)
macro_rules! Depcrate_streamimpl_1693 {
() => {
// Module: crate::stream
// Provides: {"impl_1693"}
// Dependencies: {}
impl < C , T , S > StreamOwned < C , T > where C : DerefMut + Deref < Target = ConnectionCommon < S > > , T : Read + Write , S : SideData , { # [doc = " Make a new StreamOwned taking the Connection `conn` and socket-like"] # [doc = " object `sock`.  This does not fail and does no IO."] # [doc = ""] # [doc = " This is the same as `Stream::new` except `conn` and `sock` are"] # [doc = " moved into the StreamOwned."] pub fn new (conn : C , sock : T) -> Self { Self { conn , sock } } # [doc = " Get a reference to the underlying socket"] pub fn get_ref (& self) -> & T { & self . sock } # [doc = " Get a mutable reference to the underlying socket"] pub fn get_mut (& mut self) -> & mut T { & mut self . sock } # [doc = " Extract the `conn` and `sock` parts from the `StreamOwned`"] pub fn into_parts (self) -> (C , T) { (self . conn , self . sock) } }
};
}
