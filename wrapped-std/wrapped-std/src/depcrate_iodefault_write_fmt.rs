// Generated macro for default_write_fmt (function)
macro_rules! Depcrate_iodefault_write_fmt {
() => {
// Module: crate::io
// Provides: {"default_write_fmt"}
// Dependencies: {}
pub (crate) fn default_write_fmt < W : Write + ? Sized > (this : & mut W , args : fmt :: Arguments < '_ > ,) -> Result < () > { struct Adapter < 'a , T : ? Sized + 'a > { inner : & 'a mut T , error : Result < () > , } impl < T : Write + ? Sized > fmt :: Write for Adapter < '_ , T > { fn write_str (& mut self , s : & str) -> fmt :: Result { match self . inner . write_all (s . as_bytes ()) { Ok (()) => Ok (()) , Err (e) => { self . error = Err (e) ; Err (fmt :: Error) } } } } let mut output = Adapter { inner : this , error : Ok (()) } ; match fmt :: write (& mut output , args) { Ok (()) => Ok (()) , Err (..) => { if output . error . is_err () { output . error } else { panic ! ("a formatting trait implementation returned an error when the underlying stream did not") ; } } } }
};
}
