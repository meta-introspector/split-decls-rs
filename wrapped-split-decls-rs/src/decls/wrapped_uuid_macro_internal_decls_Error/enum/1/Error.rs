use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
enum Error { NonStringLiteral , UuidParse (LitStr , error :: Error) , }
}