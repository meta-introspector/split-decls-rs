// Generated macro for read_until (function)
macro_rules! Depcrate_ioread_until {
() => {
// Module: crate::io
// Provides: {"read_until"}
// Dependencies: {}
fn read_until < R : BufRead + ? Sized > (r : & mut R , delim : u8 , buf : & mut Vec < u8 >) -> Result < usize > { let mut read = 0 ; loop { let (done , used) = { let available = match r . fill_buf () { Ok (n) => n , Err (ref e) if e . is_interrupted () => continue , Err (e) => return Err (e) , } ; match memchr :: memchr (delim , available) { Some (i) => { buf . extend_from_slice (& available [..= i]) ; (true , i + 1) } None => { buf . extend_from_slice (available) ; (false , available . len ()) } } } ; r . consume (used) ; read += used ; if done || used == 0 { return Ok (read) ; } } }
};
}
