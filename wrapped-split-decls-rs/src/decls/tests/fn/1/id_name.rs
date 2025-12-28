use serde::{Deserialize, Serialize};
use std::collections::HashMap;

fn id_name < 'a > (n : & Node) -> Id < 'a > { Id :: new (format ! ("N{}" , * n)) . unwrap () }