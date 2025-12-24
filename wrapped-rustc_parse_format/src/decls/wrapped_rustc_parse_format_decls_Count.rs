use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// A count is used for the precision and width parameters of an integer, and
/// can reference either an argument or a literal integer.
#[derive(Clone, Debug, PartialEq, Default)]
pub enum Count<'input> {
    /// The count is specified explicitly.
    CountIs(u16),
    /// The count is specified by the argument with the given name.
    CountIsName(&'input str, Range<usize>),
    /// The count is specified by the argument at the given index.
    CountIsParam(usize),
    /// The count is specified by a star (like in `{:.*}`) that refers to the argument at the given index.
    CountIsStar(usize),
    /// The count is implied and cannot be explicitly specified.
    #[default]
    CountImplied,
}
