// Generated macro for write_if (function)
macro_rules! Depcrate_formattingwrite_if {
() => {
// Module: crate::formatting
// Provides: {"write_if"}
// Dependencies: {}
# [doc = " If `pred` is true, write all bytes to the output, returning the number of bytes written."] # [inline] pub (crate) fn write_if (output : & mut (impl io :: Write + ? Sized) , pred : bool , bytes : & [u8] ,) -> io :: Result < usize > { if pred { write (output , bytes) } else { Ok (0) } }
};
}
