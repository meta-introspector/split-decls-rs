use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl IndexEntryFlag { is_bit_set ! (is_extended , IndexEntryFlag :: EXTENDED) ; is_bit_set ! (is_valid , IndexEntryFlag :: VALID) ; }