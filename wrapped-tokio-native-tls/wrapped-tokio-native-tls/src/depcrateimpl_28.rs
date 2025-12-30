// Generated macro for impl_28 (impl)
macro_rules! Depcrateimpl_28 {
() => {
// Module: crate
// Provides: {"impl_28"}
// Dependencies: {}
impl < S > TlsStream < S > { fn with_context < F , R > (& mut self , ctx : & mut Context < '_ > , f : F) -> Poll < io :: Result < R > > where F : FnOnce (& mut native_tls :: TlsStream < AllowStd < S > >) -> io :: Result < R > , AllowStd < S > : Read + Write , { self . 0 . get_mut () . context = ctx as * mut _ as * mut () ; let g = Guard (self) ; match f (& mut (g . 0) . 0) { Ok (v) => Poll :: Ready (Ok (v)) , Err (ref e) if e . kind () == io :: ErrorKind :: WouldBlock => Poll :: Pending , Err (e) => Poll :: Ready (Err (e)) , } } # [doc = " Returns a shared reference to the inner stream."] pub fn get_ref (& self) -> & native_tls :: TlsStream < AllowStd < S > > { & self . 0 } # [doc = " Returns a mutable reference to the inner stream."] pub fn get_mut (& mut self) -> & mut native_tls :: TlsStream < AllowStd < S > > { & mut self . 0 } }
};
}
