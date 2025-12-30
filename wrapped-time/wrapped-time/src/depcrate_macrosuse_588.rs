// Generated macro for use_588 (pub_use)
macro_rules! Depcrate_macrosuse_588 {
() => {
// Module: crate::macros
// Provides: {"use_588"}
// Dependencies: {}
# [doc = " Equivalent of performing [`format_description::parse()`] at compile time."] # [doc = ""] # [doc = " Using the macro instead of the function results in a static slice rather than a"] # [doc = " [`Vec`](alloc::vec::Vec), such that it can be used in `#![no_alloc]` situations."] # [doc = ""] # [doc = " The resulting expression can be used in `const` or `static` declarations, and implements"] # [doc = " the sealed traits required for both formatting and parsing."] # [cfg_attr (feature = "alloc" , doc = "```rust")] # [cfg_attr (not (feature = "alloc") , doc = "```rust,ignore")] # [doc = " # use time::{format_description, macros::format_description};"] # [doc = " assert_eq!("] # [doc = "     format_description!(\"[hour]:[minute]:[second]\"),"] # [doc = "     format_description::parse(\"[hour]:[minute]:[second]\")?"] # [doc = " );"] # [doc = " # Ok::<_, time::Error>(())"] # [doc = " ```"] # [doc = " "] # [doc = " The syntax accepted by this macro is the same as [`format_description::parse()`], which can"] # [doc = " be found in [the book](https://time-rs.github.io/book/api/format-description.html)."] # [doc = ""] # [doc = " [`format_description::parse()`]: crate::format_description::parse()"] # [cfg (any (feature = "formatting" , feature = "parsing"))] pub use time_macros :: format_description ;
};
}
