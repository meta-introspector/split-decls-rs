// Generated macro for Reader (struct)
macro_rules! Depcrate_readerReader {
() => {
// Module: crate::reader
// Provides: {"Reader"}
// Dependencies: {}
# [doc = " A read-only, forward-only cursor into the data in an `Input`."] # [doc = ""] # [doc = " Using `Reader` to parse input helps to ensure that no byte of the input"] # [doc = " will be accidentally processed more than once. Using `Reader` in"] # [doc = " conjunction with `read_all` and `read_all_optional` helps ensure that no"] # [doc = " byte of the input is accidentally left unprocessed. The methods of `Reader`"] # [doc = " never panic, so `Reader` also assists the writing of panic-free code."] # [doc = ""] # [doc = " Intentionally avoids implementing `PartialEq` and `Eq` to avoid implicit"] # [doc = " non-constant-time comparisons."] pub struct Reader < 'a > { input : no_panic :: Slice < 'a > , i : usize , }
};
}
