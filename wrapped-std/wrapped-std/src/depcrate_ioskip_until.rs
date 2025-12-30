// Generated macro for skip_until (function)
macro_rules! Depcrate_ioskip_until {
() => {
// Module: crate::io
// Provides: {"skip_until"}
// Dependencies: {}
fn skip_until < R : BufRead + ? Sized > (r : & mut R , delim : u8) -> Result < usize > { let mut read = 0 ; loop { let (done , used) = { let available = match r . fill_buf () { Ok (n) => n , Err (ref e) if e . kind () == ErrorKind :: Interrupted => continue , Err (e) => return Err (e) , } ; match memchr :: memchr (delim , available) { Some (i) => (true , i + 1) , None => (false , available . len ()) , } } ; r . consume (used) ; read += used ; if done || used == 0 { return Ok (read) ; } } }
};
}
