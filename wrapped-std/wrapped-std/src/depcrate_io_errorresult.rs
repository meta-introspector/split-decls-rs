// Generated macro for Result (type)
macro_rules! Depcrate_io_errorResult {
() => {
// Module: crate::io::error
// Provides: {"Result"}
// Dependencies: {}
# [doc = " A specialized [`Result`] type for I/O operations."] # [doc = ""] # [doc = " This type is broadly used across [`std::io`] for any operation which may"] # [doc = " produce an error."] # [doc = ""] # [doc = " This type alias is generally used to avoid writing out [`io::Error`] directly and"] # [doc = " is otherwise a direct mapping to [`Result`]."] # [doc = ""] # [doc = " While usual Rust style is to import types directly, aliases of [`Result`]"] # [doc = " often are not, to make it easier to distinguish between them. [`Result`] is"] # [doc = " generally assumed to be [`std::result::Result`][`Result`], and so users of this alias"] # [doc = " will generally use `io::Result` instead of shadowing the [prelude]'s import"] # [doc = " of [`std::result::Result`][`Result`]."] # [doc = ""] # [doc = " [`std::io`]: crate::io"] # [doc = " [`io::Error`]: Error"] # [doc = " [`Result`]: crate::result::Result"] # [doc = " [prelude]: crate::prelude"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " A convenience function that bubbles an `io::Result` to its caller:"] # [doc = ""] # [doc = " ```"] # [doc = " use std::io;"] # [doc = ""] # [doc = " fn get_string() -> io::Result<String> {"] # [doc = "     let mut buffer = String::new();"] # [doc = ""] # [doc = "     io::stdin().read_line(&mut buffer)?;"] # [doc = ""] # [doc = "     Ok(buffer)"] # [doc = " }"] # [doc = " ```"] # [stable (feature = "rust1" , since = "1.0.0")] # [doc (search_unbox)] pub type Result < T > = result :: Result < T , Error > ;
};
}
