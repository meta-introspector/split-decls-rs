// Generated macro for write_if_else (function)
macro_rules! Depcrate_formattingwrite_if_else {
() => {
// Module: crate::formatting
// Provides: {"write_if_else"}
// Dependencies: {}
# [doc = " If `pred` is true, write `true_bytes` to the output. Otherwise, write `false_bytes`."] # [inline] pub (crate) fn write_if_else (output : & mut (impl io :: Write + ? Sized) , pred : bool , true_bytes : & [u8] , false_bytes : & [u8] ,) -> io :: Result < usize > { write (output , if pred { true_bytes } else { false_bytes }) }
};
}
