// Generated macro for impl_64 (impl)
macro_rules! Depcrate_channelimpl_64 {
() => {
// Module: crate::channel
// Provides: {"impl_64"}
// Dependencies: {}
impl Read for Stream { fn read (& mut self , data : & mut [u8]) -> io :: Result < usize > { let mut locked = self . lock () ; if locked . eof () { return Ok (0) ; } let data = match locked . read_limit . as_mut () { Some (amt) => { let len = data . len () ; & mut data [.. cmp :: min (* amt as usize , len)] } None => data , } ; let ret = unsafe { let rc = raw :: libssh2_channel_read_ex (locked . raw , locked . id as c_int , data . as_mut_ptr () as * mut _ , data . len () as size_t ,) ; locked . sess . rc (rc as c_int) . map (| () | rc as usize) } ; match ret { Ok (n) => { if let Some (ref mut amt) = locked . read_limit . as_mut () { * * amt -= n as u64 ; } Ok (n) } Err (e) => Err (e . into ()) , } } }
};
}
