use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < 'tcx , I > ! TypeVisitable < I > for ClosureOutlivesSubjectTy < 'tcx > { }
}