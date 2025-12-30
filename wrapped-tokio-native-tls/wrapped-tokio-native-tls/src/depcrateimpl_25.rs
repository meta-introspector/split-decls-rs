// Generated macro for impl_25 (impl)
macro_rules! Depcrateimpl_25 {
() => {
// Module: crate
// Provides: {"impl_25"}
// Dependencies: {}
impl < S > AllowStd < S > where S : Unpin , { fn with_context < F , R > (& mut self , f : F) -> io :: Result < R > where F : FnOnce (& mut Context < '_ > , Pin < & mut S >) -> Poll < io :: Result < R > > , { unsafe { assert ! (! self . context . is_null ()) ; let waker = & mut * (self . context as * mut _) ; match f (waker , Pin :: new (& mut self . inner)) { Poll :: Ready (r) => r , Poll :: Pending => Err (io :: Error :: from (io :: ErrorKind :: WouldBlock)) , } } } }
};
}
