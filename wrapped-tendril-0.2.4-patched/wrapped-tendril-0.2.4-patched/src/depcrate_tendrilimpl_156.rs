// Generated macro for impl_156 (impl)
macro_rules! Depcrate_tendrilimpl_156 {
() => {
// Module: crate::tendril
// Provides: {"impl_156"}
// Dependencies: {}
impl < T > ReadExt for T where T : io :: Read { # [doc = " Read all bytes until EOF."] fn read_to_tendril < A > (& mut self , buf : & mut Tendril < fmt :: Bytes , A >) -> io :: Result < usize > where A : Atomicity , { const DEFAULT_BUF_SIZE : u32 = 64 * 1024 ; let start_len = buf . len () ; let mut len = start_len ; let mut new_write_size = 16 ; let ret ; loop { if len == buf . len () { if new_write_size < DEFAULT_BUF_SIZE { new_write_size *= 2 ; } unsafe { buf . push_uninitialized (new_write_size) ; } } match self . read (& mut buf [len ..]) { Ok (0) => { ret = Ok (len - start_len) ; break ; } Ok (n) => len += n , Err (ref e) if e . kind () == io :: ErrorKind :: Interrupted => { } Err (e) => { ret = Err (e) ; break ; } } } let buf_len = buf . len32 () ; buf . pop_back (buf_len - (len as u32)) ; ret } }
};
}
