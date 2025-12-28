use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Provides frequently-needed information about the diagnostic kinds being derived for this type."] # [derive (Clone , Copy , Debug)] struct KindsStatistics { has_multipart_suggestion : bool , all_multipart_suggestions : bool , has_normal_suggestion : bool , all_applicabilities_static : bool , }