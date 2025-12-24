use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// A [`FileId`] and [`Edition`] bundled up together.
/// The MSB is reserved for `HirFileId` encoding, more upper bits are used to then encode the edition.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EditionedFileId(u32);
