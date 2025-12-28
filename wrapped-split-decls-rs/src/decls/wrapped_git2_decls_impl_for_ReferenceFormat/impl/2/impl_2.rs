use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl ReferenceFormat { is_bit_set ! (is_allow_onelevel , ReferenceFormat :: ALLOW_ONELEVEL) ; is_bit_set ! (is_refspec_pattern , ReferenceFormat :: REFSPEC_PATTERN) ; is_bit_set ! (is_refspec_shorthand , ReferenceFormat :: REFSPEC_SHORTHAND) ; }