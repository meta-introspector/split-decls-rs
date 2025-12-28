use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltype! {
pub type FluentBundle = IntoDynSyncSend < fluent_bundle :: bundle :: FluentBundle < FluentResource , IntlLangMemoizer > > ;
}