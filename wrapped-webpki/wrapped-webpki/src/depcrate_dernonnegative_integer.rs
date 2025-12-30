// Generated macro for nonnegative_integer (function)
macro_rules! Depcrate_dernonnegative_integer {
() => {
// Module: crate::der
// Provides: {"nonnegative_integer"}
// Dependencies: {}
pub (crate) fn nonnegative_integer < 'a > (input : & mut untrusted :: Reader < 'a > ,) -> Result < untrusted :: Input < 'a > , Error > { let value = expect_tag (input , Tag :: Integer) ? ; match value . as_slice_less_safe () . split_first () . ok_or (Error :: BadDer) ? { (0 , rest) => { match rest . first () { None => Ok (value) , Some (& second) if second & 0x80 == 0x80 => Ok (untrusted :: Input :: from (rest)) , _ => Err (Error :: BadDer) , } } (first , _) if first & 0x80 == 0x00 => Ok (value) , (_ , _) => Err (Error :: BadDer) , } }
};
}
