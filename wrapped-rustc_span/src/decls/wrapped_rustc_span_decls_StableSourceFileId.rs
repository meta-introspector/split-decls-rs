use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// This is a [SourceFile] identifier that is used to correlate source files between
/// subsequent compilation sessions (which is something we need to do during
/// incremental compilation).
///
/// It is a hash value (so we can efficiently consume it when stable-hashing
/// spans) that consists of the `FileName` and the `StableCrateId` of the crate
/// the source file is from. The crate id is needed because sometimes the
/// `FileName` is not unique within the crate graph (think `src/lib.rs`, for
/// example).
///
/// The way the crate-id part is handled is a bit special: source files of the
/// local crate are hashed as `(filename, None)`, while source files from
/// upstream crates have a hash of `(filename, Some(stable_crate_id))`. This
/// is because SourceFiles for the local crate are allocated very early in the
/// compilation process when the `StableCrateId` is not yet known. If, due to
/// some refactoring of the compiler, the `StableCrateId` of the local crate
/// were to become available, it would be better to uniformly make this a
/// hash of `(filename, stable_crate_id)`.
///
/// When `SourceFile`s are exported in crate metadata, the `StableSourceFileId`
/// is updated to incorporate the `StableCrateId` of the exporting crate.
#[derive(
    Debug,
    Clone,
    Copy,
    Hash,
    PartialEq,
    Eq,
    HashStable_Generic,
    Encodable,
    Decodable,
    Default,
    PartialOrd,
    Ord
)]
pub struct StableSourceFileId(Hash128);
