use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: id_name");
fn id_name < 'a > (n : & Node) -> Id < 'a > { Id :: new (format ! ("N{}" , * n)) . unwrap () }
}