use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Parse tokens of source code into the chosen syntax tree node.
///
/// This is preferred over parsing a string because tokens are able to preserve
/// information about where in the user's code they were originally written (the
/// "span" of the token), possibly allowing the compiler to produce better error
/// messages.
///
/// This function parses a `proc_macro::TokenStream` which is the type used for
/// interop with the compiler in a procedural macro. To parse a
/// `proc_macro2::TokenStream`, use [`syn::parse2`] instead.
///
/// [`syn::parse2`]: parse2
///
/// This function enforces that the input is fully parsed. If there are any
/// unparsed tokens at the end of the stream, an error is returned.
#[cfg(all(feature = "parsing", feature = "proc-macro"))]
#[cfg_attr(docsrs, doc(cfg(all(feature = "parsing", feature = "proc-macro"))))]
pub fn parse<T: parse::Parse>(tokens: proc_macro::TokenStream) -> Result<T> {
    parse::Parser::parse(T::parse, tokens)
}
