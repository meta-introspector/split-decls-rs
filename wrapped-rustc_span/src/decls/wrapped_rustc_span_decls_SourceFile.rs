use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// A single source in the [`SourceMap`].
pub struct SourceFile {
    /// The name of the file that the source came from. Source that doesn't
    /// originate from files has names between angle brackets by convention
    /// (e.g., `<anon>`).
    pub name: FileName,
    /// The complete source code.
    pub src: Option<Arc<String>>,
    /// The source code's hash.
    pub src_hash: SourceFileHash,
    /// Used to enable cargo to use checksums to check if a crate is fresh rather
    /// than mtimes. This might be the same as `src_hash`, and if the requested algorithm
    /// is identical we won't compute it twice.
    pub checksum_hash: Option<SourceFileHash>,
    /// The external source code (used for external crates, which will have a `None`
    /// value as `self.src`.
    pub external_src: FreezeLock<ExternalSource>,
    /// The start position of this source in the `SourceMap`.
    pub start_pos: BytePos,
    /// The byte length of this source after normalization.
    pub normalized_source_len: RelativeBytePos,
    /// The byte length of this source before normalization.
    pub unnormalized_source_len: u32,
    /// Locations of lines beginnings in the source code.
    pub lines: FreezeLock<SourceFileLines>,
    /// Locations of multi-byte characters in the source code.
    pub multibyte_chars: Vec<MultiByteChar>,
    /// Locations of characters removed during normalization.
    pub normalized_pos: Vec<NormalizedPos>,
    /// A hash of the filename & crate-id, used for uniquely identifying source
    /// files within the crate graph and for speeding up hashing in incremental
    /// compilation.
    pub stable_id: StableSourceFileId,
    /// Indicates which crate this `SourceFile` was imported from.
    pub cnum: CrateNum,
}
