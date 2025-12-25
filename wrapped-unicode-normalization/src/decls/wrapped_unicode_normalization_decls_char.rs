use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Methods for composing and decomposing characters.
pub mod char {
    pub use crate::lookups::{canonical_combining_class, is_combining_mark};
    pub use crate::normalize::{
        compose, decompose_canonical, decompose_cjk_compat_variants, decompose_compatible,
    };
    /// Return whether the given character is assigned (`General_Category` != `Unassigned`)
    /// and not Private-Use (`General_Category` != `Private_Use`), in the supported version
    /// of Unicode.
    pub use crate::tables::is_public_assigned;
}
