use serde::{Deserialize, Serialize};
use std::collections::HashMap;

fn as_name_opt (name : Option < impl AsName >) -> Name { name . map_or_else (Name :: missing , | name | name . as_name ()) }