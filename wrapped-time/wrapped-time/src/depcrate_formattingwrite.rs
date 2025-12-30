// Generated macro for write (function)
macro_rules! Depcrate_formattingwrite {
() => {
// Module: crate::formatting
// Provides: {"write"}
// Dependencies: {}
# [doc = " Write all bytes to the output, returning the number of bytes written."] # [inline] pub (crate) fn write (output : & mut (impl io :: Write + ? Sized) , bytes : & [u8]) -> io :: Result < usize > { output . write_all (bytes) ? ; Ok (bytes . len ()) }
};
}
