use serde::{Deserialize, Serialize};
use std::collections::HashMap;

struct DiagnosticsContext < 'a > { config : & 'a DiagnosticsConfig , sema : Semantics < 'a , RootDatabase > , resolve : & 'a AssistResolveStrategy , edition : Edition , display_target : DisplayTarget , is_nightly : bool , }