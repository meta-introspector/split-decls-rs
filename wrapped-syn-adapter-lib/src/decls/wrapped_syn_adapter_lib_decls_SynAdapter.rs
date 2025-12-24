use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// A trait for parsing Rust code, abstracting different parsing implementations.
pub trait SynAdapter: Send + Sync {
    /// Parses a Rust file from a given path into a `syn::File` (or equivalent AST).
    fn parse_file(&self, path: &Path) -> Result<File>;
    /// Parses a string containing Rust code into a `syn::File` (or equivalent AST).
    fn parse_str(&self, code: &str) -> Result<File>;
    /// Returns a reference to `Any` for downcasting.
    fn as_any(&self) -> &dyn Any;
}
