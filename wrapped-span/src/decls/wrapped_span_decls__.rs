use serde::{Deserialize, Serialize};
use std::collections::HashMap;
const _: () = assert!(
    EditionedFileId::RESERVED_MASK ^ EditionedFileId::EDITION_MASK ^
    EditionedFileId::FILE_ID_MASK == 0xFFFF_FFFF
);
