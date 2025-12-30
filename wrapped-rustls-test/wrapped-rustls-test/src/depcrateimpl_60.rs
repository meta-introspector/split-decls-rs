// Generated macro for impl_60 (impl)
macro_rules! Depcrateimpl_60 {
() => {
// Module: crate
// Provides: {"impl_60"}
// Dependencies: {}
impl io :: Read for TestNonBlockIo { fn read (& mut self , buf : & mut [u8]) -> io :: Result < usize > { match self . reads . pop () { None => Err (io :: ErrorKind :: WouldBlock . into ()) , Some (data) => { assert ! (data . len () <= buf . len ()) ; let take = core :: cmp :: min (data . len () , buf . len ()) ; buf [.. take] . clone_from_slice (& data [.. take]) ; Ok (take) } } } }
};
}
