use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltrait! {
# [doc = " A trait for AST nodes that can produce a \"dummy\" or placeholder version of themselves."] # [doc = ""] # [doc = " This is primarily used for error recovery during macro expansion. If a macro fails"] # [doc = " to produce a valid AST node, a dummy node can be inserted into the AST. This allows"] # [doc = " the compiler to continue with further processing and error reporting, rather than"] # [doc = " stopping compilation entirely."] # [doc = ""] # [doc = " For n00bs: This trait gives any important piece of Rust code (like an expression,"] # [doc = " a type, or a whole program \"crate\") the ability to say: \"If I break, here's a safe,"] # [doc = " empty version of myself that the compiler can use to keep working.\""] pub trait DummyAstNode { # [doc = " Creates a default, dummy instance of the AST node."] fn dummy () -> Self ; }
}