use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl ExpandError { fn new (span : Span , kind : ExpandErrorKind) -> ExpandError { ExpandError { inner : Arc :: new ((span , kind)) , } } fn binding_error (span : Span , e : impl Into < Box < str > >) -> ExpandError { ExpandError { inner : Arc :: new ((span , ExpandErrorKind :: BindingError (Box :: new (e . into ())))) , } } }
}