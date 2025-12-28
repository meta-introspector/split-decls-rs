use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < 'tcx , I > ! TypeFoldable < I > for ClosureOutlivesSubjectTy < 'tcx > { }