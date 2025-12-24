use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Parse a string of Rust code into the chosen syntax tree node.
///
/// This function enforces that the input is fully parsed. If there are any
/// unparsed tokens at the end of the stream, an error is returned.
///
/// # Hygiene
///
/// Every span in the resulting syntax tree will be set to resolve at the macro
/// call site.
///
/// # Examples
///
/// ```
/// use syn::{Expr, Result};
///
/// fn run() -> Result<()> {
///     let code = "assert_eq!(u8::max_value(), 255)";
///     let expr = syn::parse_str::<Expr>(code)?;
///     println!("{:#?}", expr);
///     Ok(())
/// }
/// #
/// # run().unwrap();
/// ```
#[cfg(feature = "parsing")]
#[cfg_attr(docsrs, doc(cfg(feature = "parsing")))]
pub fn parse_str<T: parse::Parse>(s: &str) -> Result<T> {
    parse::Parser::parse_str(T::parse, s)
}
