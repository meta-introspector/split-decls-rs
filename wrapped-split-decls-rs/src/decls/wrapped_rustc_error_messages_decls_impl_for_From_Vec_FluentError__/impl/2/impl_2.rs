use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl From < Vec < FluentError > > for TranslationBundleError { fn from (mut errs : Vec < FluentError >) -> Self { TranslationBundleError :: AddResource (errs . pop () . expect ("failed adding resource to bundle with no errors") ,) } }
}