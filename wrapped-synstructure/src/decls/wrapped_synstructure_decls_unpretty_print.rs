use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Dumps an unpretty version of a tokenstream. Takes any type which implements
/// `Display`.
///
/// This is mostly useful for visualizing the output of a procedural macro, as
/// it makes it marginally more readable. It is used in the implementation of
/// `test_derive!` to unprettily print the output.
///
/// # Stability
///
/// The stability of the output of this function is not guaranteed. Do not
/// assert that the output of this function does not change between minor
/// versions.
///
/// # Example
///
/// ```
/// # use quote::quote;
/// assert_eq!(
///     synstructure::unpretty_print(quote! {
///         const _: () = {
///             extern crate krate;
///             impl<T, U> krate::Trait for A<T, U>
///             where
///                 Option<U>: krate::Trait,
///                 U: krate::Trait
///             {
///                 fn a() {}
///             }
///         };
///     }),
///     "const _ : (
///     )
/// = {
///     extern crate krate ;
///     impl < T , U > krate :: Trait for A < T , U > where Option < U > : krate :: Trait , U : krate :: Trait {
///         fn a (
///             )
///         {
///             }
///         }
///     }
/// ;
/// "
/// )
/// ```
pub fn unpretty_print<T: std::fmt::Display>(ts: T) -> String {
    let mut res = String::new();
    let raw_s = ts.to_string();
    let mut s = &raw_s[..];
    let mut indent = 0;
    while let Some(i) = s.find(&['(', '{', '[', ')', '}', ']', ';'][..]) {
        match &s[i..=i] {
            "(" | "{" | "[" => indent += 1,
            ")" | "}" | "]" => indent -= 1,
            _ => {}
        }
        res.push_str(&s[..=i]);
        res.push('\n');
        for _ in 0..indent {
            res.push_str("    ");
        }
        s = trim_start_matches(&s[i + 1..], ' ');
    }
    res.push_str(s);
    res
}
