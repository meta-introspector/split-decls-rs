use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < 'tcx , I > ! TypeFoldable < I > for ClosureOutlivesSubjectTy < 'tcx > { }
}