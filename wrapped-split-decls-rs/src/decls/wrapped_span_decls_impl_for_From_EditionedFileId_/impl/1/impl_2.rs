use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl From < EditionedFileId > for FileId { fn from (value : EditionedFileId) -> Self { value . file_id () } }