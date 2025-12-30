// Generated macro for impl_61 (impl)
macro_rules! Depcrateimpl_61 {
() => {
// Module: crate
// Provides: {"impl_61"}
// Dependencies: {}
impl io :: Write for TestNonBlockIo { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { match self . writes . pop () { None => Err (io :: ErrorKind :: WouldBlock . into ()) , Some (n) => Ok (core :: cmp :: min (n , buf . len ())) , } } fn flush (& mut self) -> io :: Result < () > { Ok (()) } }
};
}
