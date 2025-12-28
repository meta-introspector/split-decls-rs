use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl Clone for Ty { fn clone (& self) -> Self { ensure_sufficient_stack (| | Self { id : self . id , kind : self . kind . clone () , span : self . span , tokens : self . tokens . clone () , }) } }