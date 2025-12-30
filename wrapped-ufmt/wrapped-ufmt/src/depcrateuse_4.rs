// Generated macro for use_4 (pub_use)
macro_rules! Depcrateuse_4 {
() => {
// Module: crate
// Provides: {"use_4"}
// Dependencies: {}
# [doc = " Write formatted data into a buffer"] # [doc = ""] # [doc = " This macro accepts a format string, a list of arguments, and a 'writer'. Arguments will be"] # [doc = " formatted according to the specified format string and the result will be passed to the writer."] # [doc = " The writer must have type `[&mut] impl uWrite` or `[&mut] ufmt::Formatter<'_, impl uWrite>`. The"] # [doc = " macro returns the associated `Error` type of the `uWrite`-r."] # [doc = ""] # [doc = " The syntax is similar to [`core::write!`] but only a handful of argument types are accepted:"] # [doc = ""] # [doc = " [`core::write!`]: https://doc.rust-lang.org/core/macro.write.html"] # [doc = ""] # [doc = " - `{}` - `uDisplay`"] # [doc = " - `{:?}` - `uDebug`"] # [doc = " - `{:#?}` - \"pretty\" `uDebug`"] # [doc = ""] # [doc = " Named parameters and \"specified\" positional parameters (`{0}`) are not supported."] # [doc = ""] # [doc = " `{{` and `}}` can be used to escape braces."] pub use ufmt_macros :: uwrite ;
};
}
