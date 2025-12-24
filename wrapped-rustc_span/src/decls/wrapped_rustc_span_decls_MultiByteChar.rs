use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Identifies an offset of a multi-byte character in a `SourceFile`.
#[derive(Copy, Clone, Encodable, Decodable, Eq, PartialEq, Debug, HashStable_Generic)]
pub struct MultiByteChar {
    /// The relative offset of the character in the `SourceFile`.
    pub pos: RelativeBytePos,
    /// The number of bytes, `>= 2`.
    pub bytes: u8,
}
