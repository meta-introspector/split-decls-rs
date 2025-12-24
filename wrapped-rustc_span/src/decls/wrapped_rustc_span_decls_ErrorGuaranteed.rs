use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Useful type to use with `Result<>` indicate that an error has already
/// been reported to the user, so no need to continue checking.
///
/// The `()` field is necessary: it is non-`pub`, which means values of this
/// type cannot be constructed outside of this crate.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[derive(HashStable_Generic)]
pub struct ErrorGuaranteed(());
